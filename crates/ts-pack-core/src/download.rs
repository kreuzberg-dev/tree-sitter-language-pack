use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::error::Error;

const GITHUB_RELEASE_BASE: &str = "https://github.com/kreuzberg-dev/tree-sitter-language-pack/releases/download";

/// Manifest describing available parser downloads for a specific version.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParserManifest {
    pub version: String,
    pub platforms: HashMap<String, PlatformBundle>,
    pub languages: HashMap<String, LanguageInfo>,
    pub groups: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformBundle {
    pub url: String,
    pub sha256: String,
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageInfo {
    pub group: String,
    pub size: u64,
}

/// Manages downloading and caching of pre-built parser shared libraries.
pub struct DownloadManager {
    version: String,
    cache_dir: PathBuf,
    manifest: Mutex<Option<ParserManifest>>,
}

impl DownloadManager {
    /// Create a new download manager for the given version.
    pub fn new(version: &str) -> Result<Self, Error> {
        let cache_dir = Self::default_cache_dir(version)?;
        Ok(Self {
            version: version.to_string(),
            cache_dir,
            manifest: Mutex::new(None),
        })
    }

    /// Create a download manager with a custom cache directory.
    pub fn with_cache_dir(version: &str, cache_dir: PathBuf) -> Self {
        Self {
            version: version.to_string(),
            cache_dir,
            manifest: Mutex::new(None),
        }
    }

    /// Default cache directory: `~/.cache/tree-sitter-language-pack/v{version}/libs/`
    pub fn default_cache_dir(version: &str) -> Result<PathBuf, Error> {
        let base = dirs::cache_dir()
            .ok_or_else(|| Error::Download("Could not determine system cache directory".to_string()))?;
        Ok(base
            .join("tree-sitter-language-pack")
            .join(format!("v{version}"))
            .join("libs"))
    }

    /// Return the path to the libs cache directory.
    pub fn cache_dir(&self) -> &Path {
        &self.cache_dir
    }

    /// List languages that are already downloaded and cached.
    pub fn installed_languages(&self) -> Vec<String> {
        let mut langs = Vec::new();
        if let Ok(entries) = fs::read_dir(&self.cache_dir) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().into_owned();
                // Extract language name from library filename: libtree_sitter_<name>.so
                if let Some(lang) = Self::lang_from_lib_filename(&name) {
                    langs.push(lang);
                }
            }
        }
        langs.sort();
        langs
    }

    /// Extract language name from a shared library filename.
    ///
    /// Reverses the `c_symbol_for` mapping: e.g. `libtree_sitter_c_sharp.dylib` → `"csharp"`.
    fn lang_from_lib_filename(filename: &str) -> Option<String> {
        let name = filename.strip_prefix("lib").unwrap_or(filename);
        let name = name
            .strip_prefix("tree_sitter_")
            .or_else(|| name.strip_prefix("tree-sitter-"))?;
        let name = name
            .strip_suffix(".so")
            .or_else(|| name.strip_suffix(".dylib"))
            .or_else(|| name.strip_suffix(".dll"))?;
        // Reverse c_symbol mapping: if the stripped name is a c_symbol value,
        // return the original language name instead.
        Some(crate::registry::lang_name_for_symbol(name).to_string())
    }

    /// Ensure the specified languages are available in the cache.
    /// Downloads the platform bundle if any requested languages are missing.
    pub fn ensure_languages(&self, names: &[&str]) -> Result<(), Error> {
        let missing: Vec<&str> = names.iter().filter(|name| !self.is_cached(name)).copied().collect();

        if missing.is_empty() {
            return Ok(());
        }

        // Fetch manifest if not already loaded
        {
            let mut guard = self.manifest.lock().unwrap();
            if guard.is_none() {
                *guard = Some(self.fetch_manifest_inner()?);
            }
        }

        let guard = self.manifest.lock().unwrap();
        let manifest = guard.as_ref().expect("manifest loaded above");

        // Verify requested languages exist in manifest
        for name in &missing {
            if !manifest.languages.contains_key(*name) {
                return Err(Error::Download(format!(
                    "Language '{}' not available for download. Available groups: {:?}",
                    name,
                    manifest.groups.keys().collect::<Vec<_>>()
                )));
            }
        }

        let platform_key = Self::platform_key();
        let bundle = manifest.platforms.get(&platform_key).ok_or_else(|| {
            Error::Download(format!(
                "No pre-built parsers available for platform '{}'. Available: {:?}",
                platform_key,
                manifest.platforms.keys().collect::<Vec<_>>()
            ))
        })?;

        // Download and extract the platform bundle
        let archive_data = self.download_bundle(&bundle.url)?;

        // Verify checksum
        let actual_hash = Self::sha256_hex(&archive_data);
        if actual_hash != bundle.sha256 {
            return Err(Error::ChecksumMismatch {
                file: bundle.url.clone(),
                expected: bundle.sha256.clone(),
                actual: actual_hash,
            });
        }

        // Extract only the requested languages
        self.extract_languages(&archive_data, &missing)?;

        Ok(())
    }

    /// Ensure all languages in a named group are available.
    pub fn ensure_group(&self, group: &str) -> Result<(), Error> {
        {
            let mut guard = self.manifest.lock().unwrap();
            if guard.is_none() {
                *guard = Some(self.fetch_manifest_inner()?);
            }
        }

        let guard = self.manifest.lock().unwrap();
        let manifest = guard.as_ref().expect("manifest loaded above");
        let langs = manifest.groups.get(group).ok_or_else(|| {
            Error::Download(format!(
                "Group '{}' not found. Available: {:?}",
                group,
                manifest.groups.keys().collect::<Vec<_>>()
            ))
        })?;

        let lang_names: Vec<String> = langs.clone();
        drop(guard);
        let names: Vec<&str> = lang_names.iter().map(String::as_str).collect();
        self.ensure_languages(&names)
    }

    /// Check if a language library is already in the cache.
    fn is_cached(&self, name: &str) -> bool {
        self.lib_path(name).exists()
    }

    /// Get the expected path for a language's shared library in the cache.
    pub fn lib_path(&self, name: &str) -> PathBuf {
        let lib_name = format!("tree_sitter_{}", crate::registry::c_symbol_for(name));
        let (prefix, ext) = if cfg!(target_os = "macos") {
            ("lib", "dylib")
        } else if cfg!(target_os = "windows") {
            ("", "dll")
        } else {
            ("lib", "so")
        };
        self.cache_dir.join(format!("{prefix}{lib_name}.{ext}"))
    }

    /// Fetch the parser manifest from GitHub Releases.
    pub fn fetch_manifest(&self) -> Result<ParserManifest, Error> {
        self.fetch_manifest_inner()
    }

    /// Internal manifest fetcher (does not require &mut self).
    fn fetch_manifest_inner(&self) -> Result<ParserManifest, Error> {
        // Check for cached manifest first
        let manifest_path = self.cache_dir.parent().map(|p| p.join("manifest.json"));
        if let Some(ref path) = manifest_path
            && path.exists()
        {
            let data = fs::read_to_string(path)?;
            let manifest: ParserManifest = serde_json::from_str(&data)?;
            if manifest.version == self.version {
                return Ok(manifest);
            }
        }

        let url = format!("{}/v{}/parsers.json", GITHUB_RELEASE_BASE, self.version);

        let body = ureq::get(&url)
            .call()
            .map_err(|e| Error::Download(format!("Failed to fetch manifest from {}: {}", url, e)))?
            .into_body()
            .read_to_string()
            .map_err(|e| Error::Download(format!("Failed to read manifest body: {}", e)))?;

        let manifest: ParserManifest = serde_json::from_str(&body)?;

        // Cache the manifest
        if let Some(ref path) = manifest_path {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(path, &body)?;
        }

        Ok(manifest)
    }

    /// Download a bundle archive from the given URL.
    fn download_bundle(&self, url: &str) -> Result<Vec<u8>, Error> {
        let response = ureq::get(url)
            .call()
            .map_err(|e| Error::Download(format!("Failed to download {}: {}", url, e)))?;

        let mut data = Vec::new();
        response
            .into_body()
            .into_reader()
            .read_to_end(&mut data)
            .map_err(|e| Error::Download(format!("Failed to read download body: {}", e)))?;

        Ok(data)
    }

    /// Extract specific languages from a zstd-compressed tar archive.
    fn extract_languages(&self, archive_data: &[u8], names: &[&str]) -> Result<(), Error> {
        fs::create_dir_all(&self.cache_dir)?;

        let decoder = zstd::Decoder::new(archive_data)
            .map_err(|e| Error::Download(format!("Failed to decompress archive: {}", e)))?;
        let mut archive = tar::Archive::new(decoder);

        // Build a set of expected filenames for the requested languages
        let expected_files: HashMap<String, &str> = names
            .iter()
            .map(|name| {
                let filename = self
                    .lib_path(name)
                    .file_name()
                    .expect("lib_path always has a filename")
                    .to_string_lossy()
                    .to_string();
                (filename, *name)
            })
            .collect();

        for entry in archive
            .entries()
            .map_err(|e| Error::Download(format!("Failed to read archive entries: {}", e)))?
        {
            let mut entry = entry.map_err(|e| Error::Download(format!("Failed to read archive entry: {}", e)))?;
            let path = entry
                .path()
                .map_err(|e| Error::Download(format!("Failed to read entry path: {}", e)))?;

            let filename = path
                .file_name()
                .map(|f| f.to_string_lossy().into_owned())
                .unwrap_or_default();

            if expected_files.contains_key(&filename) {
                let dest = self.cache_dir.join(&filename);
                entry
                    .unpack(&dest)
                    .map_err(|e| Error::Download(format!("Failed to extract {}: {}", filename, e)))?;
            }
        }

        Ok(())
    }

    /// Remove all cached parser libraries.
    pub fn clean_cache(&self) -> Result<(), Error> {
        if self.cache_dir.exists() {
            fs::remove_dir_all(&self.cache_dir)?;
        }
        Ok(())
    }

    /// Compute SHA-256 hex digest.
    fn sha256_hex(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let hash = hasher.finalize();
        let mut hex = String::with_capacity(hash.len() * 2);
        for byte in hash {
            use std::fmt::Write;
            write!(hex, "{byte:02x}").unwrap();
        }
        hex
    }

    /// Platform key for the current OS/arch, e.g. "linux-x86_64", "macos-arm64".
    fn platform_key() -> String {
        let os = if cfg!(target_os = "macos") {
            "macos"
        } else if cfg!(target_os = "windows") {
            "windows"
        } else {
            "linux"
        };

        let arch = if cfg!(target_arch = "aarch64") {
            if cfg!(target_os = "macos") { "arm64" } else { "aarch64" }
        } else if cfg!(target_arch = "x86_64") {
            "x86_64"
        } else {
            std::env::consts::ARCH
        };

        format!("{os}-{arch}")
    }
}
