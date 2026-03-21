"""
Sync version from Cargo.toml workspace to all package manifests.

This script reads the version from Cargo.toml [workspace.package] and updates:
- Python pyproject.toml
- Node.js package.json (ts-pack-node)
- Elixir mix.exs
- Java pom.xml
- Ruby gemspec
- WASM Cargo.toml + package.json
"""

import json
import re
import sys
from pathlib import Path


def get_repo_root() -> Path:
    """Get the repository root directory."""
    script_dir = Path(__file__).resolve().parent
    return script_dir.parent


def get_workspace_version(repo_root: Path) -> str:
    """Extract version from Cargo.toml [workspace.package]."""
    cargo_toml = repo_root / "Cargo.toml"
    if not cargo_toml.exists():
        msg = f"Cargo.toml not found at {cargo_toml}"
        raise FileNotFoundError(msg)

    content = cargo_toml.read_text()
    match = re.search(
        r"^\[workspace\.package\]\s*\nversion\s*=\s*\"([^\"]+)\"",
        content,
        re.MULTILINE,
    )

    if not match:
        msg = "Could not find version in Cargo.toml [workspace.package]"
        raise ValueError(msg)

    return match.group(1)


def cargo_to_pep440(version: str) -> str:
    """Convert Cargo pre-release version to PEP 440 format.

    Examples: 1.0.0-rc.1 -> 1.0.0rc1, 1.0.0-alpha.2 -> 1.0.0a2, 1.0.0 -> 1.0.0
    """
    m = re.match(r"^(\d+\.\d+\.\d+)-?(rc|alpha|beta|a|b)\.?(\d+)?$", version)
    if m:
        base, pre, num = m.group(1), m.group(2), m.group(3) or "0"
        pre_map = {"alpha": "a", "beta": "b", "rc": "rc", "a": "a", "b": "b"}
        return f"{base}{pre_map.get(pre, pre)}{num}"
    return version


def update_pyproject_toml(file_path: Path, version: str) -> tuple[bool, str, str]:
    """Update pyproject.toml version field."""
    content = file_path.read_text()
    original_content = content
    match = re.search(r'^version\s*=\s*"([^"]+)"', content, re.MULTILINE)
    old_version = match.group(1) if match else "NOT FOUND"

    if old_version != version:
        content = re.sub(
            r'^(version\s*=\s*)"[^"]+"',
            rf'\1"{version}"',
            content,
            count=1,
            flags=re.MULTILINE,
        )

    if content != original_content:
        file_path.write_text(content)
        return True, old_version, version

    return False, old_version, version


def update_package_json(file_path: Path, version: str) -> tuple[bool, str, str]:
    """Update package.json version field and @kreuzberg/* optionalDependencies."""
    data = json.loads(file_path.read_text())
    old_version = data.get("version", "N/A")
    changed = False

    if data.get("version") != version:
        data["version"] = version
        changed = True

    # Also update @kreuzberg/* optionalDependencies to match
    for section in ("optionalDependencies", "dependencies"):
        deps = data.get(section, {})
        for key in list(deps):
            if key.startswith("@kreuzberg/tree-sitter-language-pack-") and deps[key] != version:
                deps[key] = version
                changed = True

    if changed:
        file_path.write_text(json.dumps(data, indent=2) + "\n")
        return True, old_version, version

    return False, old_version, version


def update_mix_exs(file_path: Path, version: str) -> tuple[bool, str, str]:
    """Update Elixir mix.exs @version attribute."""
    content = file_path.read_text()
    match = re.search(r'@version\s+"([^"]+)"', content)
    old_version = match.group(1) if match else "NOT FOUND"

    if old_version == version:
        return False, old_version, version

    new_content = re.sub(r'(@version\s+)"[^"]+"', rf'\1"{version}"', content)

    if new_content != content:
        file_path.write_text(new_content)
        return True, old_version, version

    return False, old_version, version


def update_pom_xml(file_path: Path, version: str) -> tuple[bool, str, str]:
    """Update Maven pom.xml version."""
    content = file_path.read_text()
    pattern = r"(<artifactId>tree-sitter-language-pack</artifactId>\s*\n\s*<version>)([^<]+)(</version>)"
    match = re.search(pattern, content, re.DOTALL)
    old_version = match.group(2) if match else "NOT FOUND"

    if old_version == version:
        return False, old_version, version

    new_content = re.sub(pattern, rf"\g<1>{version}\g<3>", content, flags=re.DOTALL)

    if new_content != content:
        file_path.write_text(new_content)
        return True, old_version, version

    return False, old_version, version


def update_gemspec(file_path: Path, version: str) -> tuple[bool, str, str]:
    """Update Ruby gemspec version field.

    Ruby gem versions use dots instead of hyphens for pre-release:
    1.0.0-rc.1 -> 1.0.0.rc.1
    """
    content = file_path.read_text()
    match = re.search(r"""spec\.version\s*=\s*['"]([^'"]+)['"]""", content)
    old_version = match.group(1) if match else "NOT FOUND"

    # Convert Cargo pre-release format to Ruby gem format
    gem_version = version.replace("-", ".")

    if old_version == gem_version:
        return False, old_version, gem_version

    new_content = re.sub(
        r"""(spec\.version\s*=\s*)['"][^'"]+['"]""",
        lambda m: f"{m.group(1)}'{gem_version}'",
        content,
    )

    if new_content != content:
        file_path.write_text(new_content)
        return True, old_version, gem_version

    return False, old_version, gem_version


def update_readme_config_yaml(file_path: Path, version: str) -> tuple[bool, str, str]:
    """Update version field in scripts/readme_config.yaml."""
    content = file_path.read_text()
    original_content = content
    match = re.search(r'^(\s*version:\s*")[^"]+(")', content, re.MULTILINE)
    old_version = match.group(0).split('"')[1] if match else "NOT FOUND"

    if old_version != version:
        content = re.sub(
            r'^(\s*version:\s*")[^"]+(")',
            rf"\g<1>{version}\g<2>",
            content,
            count=1,
            flags=re.MULTILINE,
        )

    if content != original_content:
        file_path.write_text(content)
        return True, old_version, version

    return False, old_version, version


def update_rust_version_const(file_path: Path, version: str) -> tuple[bool, str, str]:
    """Update VERSION const in Rust source files."""
    content = file_path.read_text()
    original_content = content
    match = re.search(r'const VERSION:\s*&str\s*=\s*"([^"]+)"', content)
    old_version = match.group(1) if match else "NOT FOUND"

    if old_version != version:
        content = re.sub(
            r'(const VERSION:\s*&str\s*=\s*")[^"]+(")',
            rf"\g<1>{version}\g<2>",
            content,
        )

    if content != original_content:
        file_path.write_text(content)
        return True, old_version, version

    return False, old_version, version


def update_test_app_version_json(file_path: Path, version: str) -> tuple[bool, str, str]:
    """Update version field in a test app package.json (the app's own version)."""
    data = json.loads(file_path.read_text())
    old_version = data.get("version", "N/A")

    if old_version == version:
        return False, old_version, version

    data["version"] = version
    file_path.write_text(json.dumps(data, indent=2) + "\n")
    return True, old_version, version


def update_test_app_version_toml(file_path: Path, version: str) -> tuple[bool, str, str]:
    """Update version field in a test app Cargo.toml (the app's own version)."""
    content = file_path.read_text()
    original_content = content
    match = re.search(r'^version\s*=\s*"([^"]+)"', content, re.MULTILINE)
    old_version = match.group(1) if match else "NOT FOUND"

    if old_version != version:
        content = re.sub(
            r'^(version\s*=\s*)"[^"]+"',
            rf'\1"{version}"',
            content,
            count=1,
            flags=re.MULTILINE,
        )

    if content != original_content:
        file_path.write_text(content)
        return True, old_version, version

    return False, old_version, version


def update_napi_index_js(file_path: Path, version: str) -> tuple[bool, str, str]:
    """Update hardcoded version strings in NAPI-RS auto-generated index.js."""
    content = file_path.read_text()
    original_content = content

    # Find current hardcoded version (first occurrence of the version check pattern)
    match = re.search(
        r'bindingPackageVersion !== "([^"]+)"',
        content,
    )
    old_version = match.group(1) if match else "NOT FOUND"

    if old_version == version:
        return False, old_version, version

    # Replace all occurrences of the old version with the new version
    content = content.replace(f'"{old_version}"', f'"{version}"')

    if content != original_content:
        file_path.write_text(content)
        return True, old_version, version

    return False, old_version, version


def update_composer_json(file_path: Path, version: str) -> tuple[bool, str, str]:
    """Update Composer composer.json version field."""
    content = json.loads(file_path.read_text())
    old_version = content.get("version", "NOT SET")

    if old_version == version:
        return False, old_version, version

    content["version"] = version
    file_path.write_text(json.dumps(content, indent=4, ensure_ascii=False) + "\n")
    return True, old_version, version


def update_csproj(file_path: Path, version: str) -> tuple[bool, str, str]:
    """Update .NET .csproj Version property."""
    content = file_path.read_text()
    match = re.search(r"<Version>([^<]+)</Version>", content)
    old_version = match.group(1) if match else "NOT FOUND"

    if old_version == version:
        return False, old_version, version

    new_content = re.sub(r"<Version>[^<]+</Version>", f"<Version>{version}</Version>", content)

    if new_content != content:
        file_path.write_text(new_content)
        return True, old_version, version

    return False, old_version, version


def update_cargo_toml_version(file_path: Path, version: str) -> tuple[bool, str, str]:
    """Update version in a non-workspace Cargo.toml (e.g. WASM crate)."""
    content = file_path.read_text()
    original_content = content

    # Update package version
    match = re.search(r'^version\s*=\s*"([^"]+)"', content, re.MULTILINE)
    old_version = match.group(1) if match else "NOT FOUND"

    if old_version != version:
        content = re.sub(
            r'^(version\s*=\s*)"[^"]+"',
            rf'\1"{version}"',
            content,
            count=1,
            flags=re.MULTILINE,
        )

    # Also update ts-pack-core version reference
    content = re.sub(
        r'(ts-pack-core\s*=\s*\{[^}]*version\s*=\s*)"[^"]+"',
        rf'\1"{version}"',
        content,
    )

    if content != original_content:
        file_path.write_text(content)
        return True, old_version, version

    return False, old_version, version


def update_gemfile(file_path: Path, version: str) -> tuple[bool, str, str]:
    """Update Ruby gem version in Gemfile.

    Handles format: gem 'tree_sitter_language_pack', '1.0.0.rc.1'
    Ruby gem versions use dots instead of hyphens for pre-release.
    """
    content = file_path.read_text()
    original_content = content
    gem_version = version.replace("-", ".")

    match = re.search(
        r"""gem\s+["']tree_sitter_language_pack["'],\s+["']([^"']+)["']""",
        content,
    )
    old_version = match.group(1) if match else "NOT FOUND"

    if old_version != gem_version:
        content = re.sub(
            r"""(gem\s+["']tree_sitter_language_pack["'],\s+["'])[^"']+(['"])""",
            lambda m: f"{m.group(1)}{gem_version}{m.group(2)}",
            content,
        )

    if content != original_content:
        file_path.write_text(content)
        return True, old_version, gem_version

    return False, old_version, gem_version


def update_gemfile_lock(file_path: Path, version: str) -> tuple[bool, str, str]:
    """Update Ruby gem version in Gemfile.lock for path gems.

    Handles the PATH/specs section where the gem version appears.
    Ruby gem versions use dots instead of hyphens for pre-release.
    """
    content = file_path.read_text()
    original_content = content
    gem_version = version.replace("-", ".")

    match = re.search(
        r"tree_sitter_language_pack\s+\(([\d.]+[\w.]*)\)",
        content,
    )
    old_version = match.group(1) if match else "NOT FOUND"

    if old_version != gem_version:
        content = re.sub(
            r"(tree_sitter_language_pack\s+\()[\d.]+[\w.]*(\))",
            lambda m: f"{m.group(1)}{gem_version}{m.group(2)}",
            content,
        )

    if content != original_content:
        file_path.write_text(content)
        return True, old_version, gem_version

    return False, old_version, gem_version


def update_go_mod_require(file_path: Path, version: str) -> tuple[bool, str, str]:
    """Update Go require directive version in go.mod."""
    content = file_path.read_text()
    original_content = content

    match = re.search(
        r"require\s+\S+\s+v([\d.]+[-\w.]*)",
        content,
    )
    old_version = match.group(1) if match else "NOT FOUND"

    if old_version != version:
        content = re.sub(
            r"(require\s+\S+\s+v)[\d.]+[-\w.]*",
            lambda m: f"{m.group(1)}{version}",
            content,
        )

    if content != original_content:
        file_path.write_text(content)
        return True, old_version, version

    return False, old_version, version


def update_cargo_dep_version(file_path: Path, version: str) -> tuple[bool, str, str]:
    """Update tree-sitter-language-pack dependency version in a Cargo.toml."""
    content = file_path.read_text()
    original_content = content

    match = re.search(
        r'tree-sitter-language-pack\s*=\s*"([^"]+)"',
        content,
    )
    old_version = match.group(1) if match else "NOT FOUND"

    if old_version != version:
        content = re.sub(
            r'(tree-sitter-language-pack\s*=\s*")[^"]+(")',
            lambda m: f"{m.group(1)}{version}{m.group(2)}",
            content,
        )

    if content != original_content:
        file_path.write_text(content)
        return True, old_version, version

    return False, old_version, version


def update_pyproject_dep_version(file_path: Path, version: str) -> tuple[bool, str, str]:
    """Update tree-sitter-language-pack dependency version in pyproject.toml (PEP 440)."""
    content = file_path.read_text()
    original_content = content
    pep_version = cargo_to_pep440(version)

    match = re.search(
        r"tree-sitter-language-pack==([\w.]+)",
        content,
    )
    old_version = match.group(1) if match else "NOT FOUND"

    if old_version != pep_version:
        content = re.sub(
            r"(tree-sitter-language-pack==)[\w.]+",
            lambda m: f"{m.group(1)}{pep_version}",
            content,
        )

    if content != original_content:
        file_path.write_text(content)
        return True, old_version, pep_version

    return False, old_version, pep_version


def update_package_json_dep(file_path: Path, version: str) -> tuple[bool, str, str]:
    """Update @kreuzberg/* dependency versions in package.json."""
    data = json.loads(file_path.read_text())
    file_path.read_text()
    changed = False

    for section in ("dependencies", "devDependencies"):
        deps = data.get(section, {})
        for key in list(deps):
            if key.startswith("@kreuzberg/") and deps[key] != version:
                deps[key] = version
                changed = True

    old_version = "unknown"
    for dep in data.get("dependencies", {}).values():
        old_version = dep
        break

    if changed:
        file_path.write_text(json.dumps(data, indent=2) + "\n")
        return True, old_version, version

    return False, version, version


def update_mix_exs_dep(file_path: Path, version: str) -> tuple[bool, str, str]:
    """Update tree_sitter_language_pack dep version in mix.exs."""
    content = file_path.read_text()
    original_content = content

    match = re.search(
        r""":tree_sitter_language_pack,\s*["']~>\s*([\d.]+[-\w.]*)["']""",
        content,
    )
    old_version = match.group(1) if match else "NOT FOUND"

    if old_version != version:
        content = re.sub(
            r"""(:tree_sitter_language_pack,\s*["']~>\s*)[\d.]+[-\w.]*""",
            lambda m: f"{m.group(1)}{version}",
            content,
        )

    if content != original_content:
        file_path.write_text(content)
        return True, old_version, version

    return False, old_version, version


def update_pom_xml_dep(file_path: Path, version: str) -> tuple[bool, str, str]:
    """Update tree-sitter-language-pack dependency version in pom.xml."""
    return update_pom_xml(file_path, version)


def update_composer_json_dep(file_path: Path, version: str) -> tuple[bool, str, str]:
    """Update kreuzberg/tree-sitter-language-pack require version in composer.json."""
    data = json.loads(file_path.read_text())
    require = data.get("require", {})
    pkg = "kreuzberg/tree-sitter-language-pack"
    old_version = require.get(pkg, "NOT FOUND")

    if old_version == version:
        return False, old_version, version

    require[pkg] = version
    data["require"] = require
    file_path.write_text(json.dumps(data, indent=4, ensure_ascii=False) + "\n")
    return True, old_version, version


def update_csproj_dep(file_path: Path, version: str) -> tuple[bool, str, str]:
    """Update TreeSitterLanguagePack PackageReference version in .csproj."""
    content = file_path.read_text()
    original_content = content

    match = re.search(
        r'PackageReference\s+Include="TreeSitterLanguagePack"\s+Version="([^"]+)"',
        content,
    )
    old_version = match.group(1) if match else "NOT FOUND"

    if old_version != version:
        content = re.sub(
            r'(PackageReference\s+Include="TreeSitterLanguagePack"\s+Version=")[^"]+(")',
            lambda m: f"{m.group(1)}{version}{m.group(2)}",
            content,
        )

    if content != original_content:
        file_path.write_text(content)
        return True, old_version, version

    return False, old_version, version


def main() -> None:
    repo_root = get_repo_root()

    try:
        version = get_workspace_version(repo_root)
    except (FileNotFoundError, ValueError) as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)

    print(f"\nSyncing version {version} from Cargo.toml\n")

    updated_files: list[str] = []
    unchanged_files: list[str] = []

    targets: list[tuple[Path, str]] = [
        (repo_root / "pyproject.toml", "pyproject"),
        (repo_root / "crates/ts-pack-python/pyproject.toml", "pyproject_pep440"),
        (repo_root / "crates/ts-pack-node/package.json", "package_json"),
        (repo_root / "crates/ts-pack-wasm/package.json", "package_json"),
        (repo_root / "crates/ts-pack-elixir/mix.exs", "mix_exs"),
        (repo_root / "crates/ts-pack-elixir/lib/tree_sitter_language_pack.ex", "mix_exs"),
        (repo_root / "crates/ts-pack-java/pom.xml", "pom_xml"),
        (repo_root / "crates/ts-pack-ruby/tree_sitter_language_pack.gemspec", "gemspec"),
        (repo_root / "crates/ts-pack-ruby/Gemfile.lock", "gemfile_lock"),
        (repo_root / "crates/ts-pack-node/index.js", "napi_index_js"),
        (repo_root / "crates/ts-pack-wasm/Cargo.toml", "cargo_toml_version"),
        (repo_root / "composer.json", "composer_json"),
        (repo_root / "packages/php/composer.json", "composer_json"),
        (repo_root / "packages/csharp/TreeSitterLanguagePack/TreeSitterLanguagePack.csproj", "csproj"),
        # Node.js platform-specific packages
        (repo_root / "crates/ts-pack-node/npm/linux-x64-gnu/package.json", "package_json"),
        (repo_root / "crates/ts-pack-node/npm/linux-arm64-gnu/package.json", "package_json"),
        (repo_root / "crates/ts-pack-node/npm/darwin-arm64/package.json", "package_json"),
        (repo_root / "crates/ts-pack-node/npm/win32-x64-msvc/package.json", "package_json"),
        # Test app manifests — update dependency versions (not package versions)
        (repo_root / "tests/test_apps/rust/Cargo.toml", "cargo_dep"),
        (repo_root / "tests/test_apps/python/pyproject.toml", "pyproject_dep"),
        (repo_root / "tests/test_apps/node/package.json", "package_json_dep"),
        (repo_root / "tests/test_apps/wasm/package.json", "package_json_dep"),
        (repo_root / "tests/test_apps/ruby/Gemfile", "gemfile"),
        (repo_root / "tests/test_apps/go/go.mod", "go_mod_require"),
        (repo_root / "tests/test_apps/java/pom.xml", "pom_xml_dep"),
        (repo_root / "tests/test_apps/elixir/mix.exs", "mix_exs_dep"),
        (repo_root / "tests/test_apps/php/composer.json", "composer_json_dep"),
        (repo_root / "tests/test_apps/csharp/TestApp.csproj", "csproj_dep"),
        # Additional version references
        (repo_root / "scripts/readme_config.yaml", "readme_config_yaml"),
        (repo_root / "tests/test_apps/rust/src/main.rs", "rust_version_const"),
        # Test app own-version fields
        (repo_root / "tests/test_apps/node/package.json", "test_app_version_json"),
        (repo_root / "tests/test_apps/wasm/package.json", "test_app_version_json"),
        (repo_root / "tests/test_apps/rust/Cargo.toml", "test_app_version_toml"),
    ]

    def update_pyproject_pep440(file_path: Path, version: str) -> tuple[bool, str, str]:
        """Update pyproject.toml version field with PEP 440 conversion."""
        pep_version = cargo_to_pep440(version)
        return update_pyproject_toml(file_path, pep_version)

    update_funcs = {
        "pyproject": update_pyproject_toml,
        "pyproject_pep440": update_pyproject_pep440,
        "package_json": update_package_json,
        "mix_exs": update_mix_exs,
        "pom_xml": update_pom_xml,
        "gemspec": update_gemspec,
        "napi_index_js": update_napi_index_js,
        "cargo_toml_version": update_cargo_toml_version,
        "composer_json": update_composer_json,
        "csproj": update_csproj,
        "gemfile": update_gemfile,
        "gemfile_lock": update_gemfile_lock,
        "go_mod_require": update_go_mod_require,
        "cargo_dep": update_cargo_dep_version,
        "pyproject_dep": update_pyproject_dep_version,
        "package_json_dep": update_package_json_dep,
        "mix_exs_dep": update_mix_exs_dep,
        "pom_xml_dep": update_pom_xml_dep,
        "composer_json_dep": update_composer_json_dep,
        "csproj_dep": update_csproj_dep,
        "readme_config_yaml": update_readme_config_yaml,
        "rust_version_const": update_rust_version_const,
        "test_app_version_json": update_test_app_version_json,
        "test_app_version_toml": update_test_app_version_toml,
    }

    for file_path, file_type in targets:
        if not file_path.exists():
            continue

        update_func = update_funcs[file_type]
        changed, old_ver, new_ver = update_func(file_path, version)
        rel_path = file_path.relative_to(repo_root)

        if changed:
            print(f"  {rel_path}: {old_ver} -> {new_ver}")
            updated_files.append(str(rel_path))
        else:
            unchanged_files.append(str(rel_path))

    print("\nSummary:")
    print(f"  Updated: {len(updated_files)} files")
    print(f"  Unchanged: {len(unchanged_files)} files")

    if updated_files:
        print(f"\nVersion sync complete! All files now at {version}\n")
    else:
        print(f"\nAll files already at {version}\n")


if __name__ == "__main__":
    main()
