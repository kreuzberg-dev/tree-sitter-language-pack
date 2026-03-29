using System;
using System.Collections.Generic;
using System.Diagnostics.CodeAnalysis;
using System.IO;
using System.Reflection;
using System.Runtime.InteropServices;
using System.Runtime.CompilerServices;

namespace TreeSitterLanguagePack;

/// <summary>
/// P/Invoke declarations for the ts_pack_ffi native library.
/// </summary>
internal static partial class NativeMethods
{
    private const string LibraryName = "ts_pack_ffi";

    /// <summary>
    /// Lazy-initialized cache for the native library handle.
    /// Uses ExecutionAndPublication mode to ensure thread-safe, one-time initialization.
    /// </summary>
    private static readonly Lazy<IntPtr> LibraryHandle =
        new(() => LoadNativeLibrary(), LazyThreadSafetyMode.ExecutionAndPublication);

    [ModuleInitializer]
    [SuppressMessage("Usage", "CA2255:The 'ModuleInitializer' attribute should not be used in libraries",
        Justification = "Required for native library resolution before P/Invoke calls.")]
    internal static void InitResolver()
    {
        NativeLibrary.SetDllImportResolver(typeof(NativeMethods).Assembly, ResolveLibrary);
    }

    // -----------------------------------------------------------------------
    // Registry lifecycle
    // -----------------------------------------------------------------------

    /// <summary>
    /// Create a new language registry. The caller must free it with <see cref="RegistryFree"/>.
    /// </summary>
    [DllImport(LibraryName, EntryPoint = "ts_pack_registry_new", CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr RegistryNew();

    /// <summary>
    /// Free a registry created with <see cref="RegistryNew"/>. Passing <see cref="IntPtr.Zero"/> is a safe no-op.
    /// </summary>
    [DllImport(LibraryName, EntryPoint = "ts_pack_registry_free", CallingConvention = CallingConvention.Cdecl)]
    internal static extern void RegistryFree(IntPtr registry);

    // -----------------------------------------------------------------------
    // Registry queries
    // -----------------------------------------------------------------------

    /// <summary>
    /// Return the number of available languages in the registry.
    /// </summary>
    [DllImport(LibraryName, EntryPoint = "ts_pack_language_count", CallingConvention = CallingConvention.Cdecl)]
    internal static extern UIntPtr LanguageCount(IntPtr registry);

    /// <summary>
    /// Get the language name at the given index. The caller must free the returned string
    /// with <see cref="FreeString"/>.
    /// </summary>
    [DllImport(LibraryName, EntryPoint = "ts_pack_language_name_at", CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr LanguageNameAt(IntPtr registry, UIntPtr index);

    /// <summary>
    /// Check whether the registry contains a language with the given name.
    /// </summary>
    [DllImport(LibraryName, EntryPoint = "ts_pack_has_language", CallingConvention = CallingConvention.Cdecl)]
    [return: MarshalAs(UnmanagedType.I1)]
    internal static extern bool HasLanguage(IntPtr registry, IntPtr name);

    /// <summary>
    /// Detect language name from a file path. Returns IntPtr.Zero if not recognized.
    /// Caller must free the result with FreeString.
    /// </summary>
    [DllImport(LibraryName, EntryPoint = "ts_pack_detect_language", CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr DetectLanguage(IntPtr path);

    /// <summary>
    /// Detect language name from file content (shebang-based). Returns IntPtr.Zero if not recognized.
    /// Caller must free the result with FreeString.
    /// </summary>
    [DllImport(LibraryName, EntryPoint = "ts_pack_detect_language_from_content", CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr DetectLanguageFromContent(IntPtr content);

    /// <summary>
    /// Detect language name from a bare file extension (without leading dot).
    /// Returns IntPtr.Zero if not recognized. Caller must free the result with FreeString.
    /// </summary>
    [DllImport(LibraryName, EntryPoint = "ts_pack_detect_language_from_extension", CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr DetectLanguageFromExtension(IntPtr ext);

    /// <summary>
    /// Detect language name from a file path based on its extension.
    /// Returns IntPtr.Zero if not recognized. Caller must free the result with FreeString.
    /// </summary>
    [DllImport(LibraryName, EntryPoint = "ts_pack_detect_language_from_path", CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr DetectLanguageFromPath(IntPtr path);

    /// <summary>
    /// Returns extension ambiguity information as a JSON C string, or IntPtr.Zero if not ambiguous.
    /// Caller must free the result with FreeString.
    /// </summary>
    [DllImport(LibraryName, EntryPoint = "ts_pack_extension_ambiguity", CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr ExtensionAmbiguity(IntPtr ext);

    /// <summary>
    /// Returns the bundled highlights query for the language, or IntPtr.Zero if unavailable.
    /// Caller must free the result with FreeString.
    /// </summary>
    [DllImport(LibraryName, EntryPoint = "ts_pack_get_highlights_query", CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr GetHighlightsQuery(IntPtr language);

    /// <summary>
    /// Returns the bundled injections query for the language, or IntPtr.Zero if unavailable.
    /// Caller must free the result with FreeString.
    /// </summary>
    [DllImport(LibraryName, EntryPoint = "ts_pack_get_injections_query", CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr GetInjectionsQuery(IntPtr language);

    /// <summary>
    /// Returns the bundled locals query for the language, or IntPtr.Zero if unavailable.
    /// Caller must free the result with FreeString.
    /// </summary>
    [DllImport(LibraryName, EntryPoint = "ts_pack_get_locals_query", CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr GetLocalsQuery(IntPtr language);

    /// <summary>
    /// Get a raw TSLanguage pointer for the given language name.
    /// Returns <see cref="IntPtr.Zero"/> on error.
    /// </summary>
    [DllImport(LibraryName, EntryPoint = "ts_pack_get_language", CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr GetLanguage(IntPtr registry, IntPtr name);

    // -----------------------------------------------------------------------
    // Parsing
    // -----------------------------------------------------------------------

    /// <summary>
    /// Parse source code and return an opaque tree handle.
    /// The caller must free the tree with <see cref="TreeFree"/>.
    /// </summary>
    [DllImport(LibraryName, EntryPoint = "ts_pack_parse_string", CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr ParseString(IntPtr registry, IntPtr name, IntPtr source, UIntPtr sourceLen);

    /// <summary>
    /// Free a tree created with <see cref="ParseString"/>. Passing <see cref="IntPtr.Zero"/> is a safe no-op.
    /// </summary>
    [DllImport(LibraryName, EntryPoint = "ts_pack_tree_free", CallingConvention = CallingConvention.Cdecl)]
    internal static extern void TreeFree(IntPtr tree);

    /// <summary>
    /// Get the type name of the root node. Caller must free with <see cref="FreeString"/>.
    /// </summary>
    [DllImport(LibraryName, EntryPoint = "ts_pack_tree_root_node_type", CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr TreeRootNodeType(IntPtr tree);

    /// <summary>
    /// Get the number of named children of the root node.
    /// </summary>
    [DllImport(LibraryName, EntryPoint = "ts_pack_tree_root_child_count", CallingConvention = CallingConvention.Cdecl)]
    internal static extern uint TreeRootChildCount(IntPtr tree);

    /// <summary>
    /// Check whether the tree contains a node with the given type name.
    /// </summary>
    [DllImport(LibraryName, EntryPoint = "ts_pack_tree_contains_node_type", CallingConvention = CallingConvention.Cdecl)]
    [return: MarshalAs(UnmanagedType.I1)]
    internal static extern bool TreeContainsNodeType(IntPtr tree, IntPtr nodeType);

    /// <summary>
    /// Check whether the tree contains any ERROR or MISSING nodes.
    /// </summary>
    [DllImport(LibraryName, EntryPoint = "ts_pack_tree_has_error_nodes", CallingConvention = CallingConvention.Cdecl)]
    [return: MarshalAs(UnmanagedType.I1)]
    internal static extern bool TreeHasErrorNodes(IntPtr tree);

    /// <summary>
    /// Return the S-expression representation of the tree. Caller must free with <see cref="FreeString"/>.
    /// </summary>
    [DllImport(LibraryName, EntryPoint = "ts_pack_tree_to_sexp", CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr TreeToSexp(IntPtr tree);

    /// <summary>
    /// Return the count of ERROR and MISSING nodes in the tree.
    /// </summary>
    [DllImport(LibraryName, EntryPoint = "ts_pack_tree_error_count", CallingConvention = CallingConvention.Cdecl)]
    internal static extern UIntPtr TreeErrorCount(IntPtr tree);

    // -----------------------------------------------------------------------
    // Process (unified API)
    // -----------------------------------------------------------------------

    /// <summary>
    /// Process source code and return a JSON string with analysis results.
    /// Caller must free the returned string with <see cref="FreeString"/>.
    /// </summary>
    [DllImport(LibraryName, EntryPoint = "ts_pack_process", CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr Process(IntPtr registry, IntPtr source, UIntPtr sourceLen, IntPtr configJson);

    // -----------------------------------------------------------------------
    // Error handling
    // -----------------------------------------------------------------------

    /// <summary>
    /// Get the last error message, or null if no error occurred.
    /// The caller must NOT free this pointer.
    /// </summary>
    [DllImport(LibraryName, EntryPoint = "ts_pack_last_error", CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr LastError();

    /// <summary>
    /// Clear the last error.
    /// </summary>
    [DllImport(LibraryName, EntryPoint = "ts_pack_clear_error", CallingConvention = CallingConvention.Cdecl)]
    internal static extern void ClearError();

    // -----------------------------------------------------------------------
    // Memory management
    // -----------------------------------------------------------------------

    /// <summary>
    /// Free a string allocated by the native library.
    /// </summary>
    [DllImport(LibraryName, EntryPoint = "ts_pack_free_string", CallingConvention = CallingConvention.Cdecl)]
    internal static extern void FreeString(IntPtr ptr);

    /// <summary>
    /// Free a string array allocated by the native library.
    /// </summary>
    [DllImport(LibraryName, EntryPoint = "ts_pack_free_string_array", CallingConvention = CallingConvention.Cdecl)]
    internal static extern void FreeStringArray(IntPtr arr);

    // -----------------------------------------------------------------------
    // Configuration and download
    // -----------------------------------------------------------------------

    /// <summary>
    /// Initialize the language pack with configuration.
    /// </summary>
    [DllImport(LibraryName, EntryPoint = "ts_pack_init", CallingConvention = CallingConvention.Cdecl)]
    internal static extern int Init(IntPtr configJson);

    /// <summary>
    /// Configure the language pack cache directory without downloading.
    /// </summary>
    [DllImport(LibraryName, EntryPoint = "ts_pack_configure", CallingConvention = CallingConvention.Cdecl)]
    internal static extern int Configure(IntPtr configJson);

    /// <summary>
    /// Download specific languages to the cache.
    /// </summary>
    [DllImport(LibraryName, EntryPoint = "ts_pack_download", CallingConvention = CallingConvention.Cdecl)]
    internal static extern int Download(IntPtr names, UIntPtr count);

    /// <summary>
    /// Download all available languages from the remote manifest.
    /// </summary>
    [DllImport(LibraryName, EntryPoint = "ts_pack_download_all", CallingConvention = CallingConvention.Cdecl)]
    internal static extern int DownloadAll();

    /// <summary>
    /// Get all language names available in the remote manifest.
    /// </summary>
    [DllImport(LibraryName, EntryPoint = "ts_pack_manifest_languages", CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr ManifestLanguages(out UIntPtr outCount);

    /// <summary>
    /// Get all languages that are already downloaded and cached locally.
    /// </summary>
    [DllImport(LibraryName, EntryPoint = "ts_pack_downloaded_languages", CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr DownloadedLanguages(out UIntPtr outCount);

    /// <summary>
    /// Delete all cached parser shared libraries.
    /// </summary>
    [DllImport(LibraryName, EntryPoint = "ts_pack_clean_cache", CallingConvention = CallingConvention.Cdecl)]
    internal static extern int CleanCache();

    /// <summary>
    /// Get the effective cache directory path as a C string.
    /// The caller must free the returned string with <see cref="FreeString"/>.
    /// </summary>
    [DllImport(LibraryName, EntryPoint = "ts_pack_cache_dir", CallingConvention = CallingConvention.Cdecl)]
    internal static extern IntPtr CacheDir();

    // -----------------------------------------------------------------------
    // Library resolution
    // -----------------------------------------------------------------------

    private static IntPtr ResolveLibrary(string libraryName, Assembly assembly, DllImportSearchPath? searchPath)
    {
        if (!string.Equals(libraryName, LibraryName, StringComparison.Ordinal))
        {
            return IntPtr.Zero;
        }

        var effectiveSearchPath = searchPath ?? DllImportSearchPath.AssemblyDirectory;
        if (NativeLibrary.TryLoad(libraryName, assembly, effectiveSearchPath, out var defaultHandle))
        {
            return defaultHandle;
        }

        return LibraryHandle.Value;
    }

    private static IntPtr LoadNativeLibrary()
    {
        var fileName = GetLibraryFileName();
        var probePaths = GetProbePaths(fileName).ToList();

        foreach (var path in probePaths)
        {
            if (NativeLibrary.TryLoad(path, out var handle))
            {
                return handle;
            }
        }

        var pathsStr = string.Join(", ", probePaths);
        throw new DllNotFoundException(
            $"Unable to locate {fileName}. Checked: {pathsStr}. " +
            "Set TSPACK_FFI_DIR or place the library in target/release.");
    }

    private static IEnumerable<string> GetProbePaths(string fileName)
    {
        var envDir = Environment.GetEnvironmentVariable("TSPACK_FFI_DIR");
        if (!string.IsNullOrWhiteSpace(envDir))
        {
            yield return Path.Combine(envDir, fileName);
        }

        yield return Path.Combine(AppContext.BaseDirectory, fileName);

        var rid = GetStableRuntimeIdentifier();
        if (!string.IsNullOrWhiteSpace(rid))
        {
            yield return Path.Combine(AppContext.BaseDirectory, "runtimes", rid!, "native", fileName);
        }

        var cwd = Directory.GetCurrentDirectory();
        yield return Path.Combine(cwd, fileName);

        var cwdRelease = Path.Combine(cwd, "target", "release", fileName);
        if (File.Exists(cwdRelease))
        {
            yield return cwdRelease;
        }

        var cwdDebug = Path.Combine(cwd, "target", "debug", fileName);
        if (File.Exists(cwdDebug))
        {
            yield return cwdDebug;
        }

        string? dir = AppContext.BaseDirectory;
        for (var i = 0; i < 5 && dir != null; i++)
        {
            var release = Path.Combine(dir, "target", "release", fileName);
            if (File.Exists(release))
            {
                yield return release;
            }

            var debugPath = Path.Combine(dir, "target", "debug", fileName);
            if (File.Exists(debugPath))
            {
                yield return debugPath;
            }

            dir = Directory.GetParent(dir)?.FullName;
        }
    }

    private static string? GetStableRuntimeIdentifier()
    {
        var arch = RuntimeInformation.ProcessArchitecture switch
        {
            Architecture.X64 => "x64",
            Architecture.Arm64 => "arm64",
            _ => null,
        };

        if (arch is null)
        {
            return null;
        }

        if (OperatingSystem.IsWindows())
        {
            return $"win-{arch}";
        }

        if (OperatingSystem.IsMacOS())
        {
            return $"osx-{arch}";
        }

        if (OperatingSystem.IsLinux())
        {
            return $"linux-{arch}";
        }

        return null;
    }

    private static string GetLibraryFileName()
    {
        if (OperatingSystem.IsWindows())
        {
            return "ts_pack_ffi.dll";
        }

        if (OperatingSystem.IsMacOS())
        {
            return "libts_pack_ffi.dylib";
        }

        return "libts_pack_ffi.so";
    }
}
