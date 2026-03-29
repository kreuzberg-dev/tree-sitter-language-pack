using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;
using System.Text.Json;

namespace TreeSitterLanguagePack;

/// <summary>
/// High-level static API for the tree-sitter language pack.
/// Manages a shared registry instance and provides convenient methods for
/// querying languages and processing source code.
/// </summary>
public static class TsPackClient
{
    private static readonly Lazy<IntPtr> SharedRegistry =
        new(() =>
        {
            var reg = NativeMethods.RegistryNew();
            if (reg == IntPtr.Zero)
            {
                InteropUtilities.ThrowIfError();
                throw new TsPackException("failed to create registry");
            }
            return reg;
        }, LazyThreadSafetyMode.ExecutionAndPublication);

    private static IntPtr Registry => SharedRegistry.Value;

    /// <summary>
    /// Get the list of all available language names.
    /// </summary>
    public static string[] AvailableLanguages()
    {
        var count = (int)(nuint)NativeMethods.LanguageCount(Registry);
        var result = new string[count];

        for (var i = 0; i < count; i++)
        {
            var namePtr = NativeMethods.LanguageNameAt(Registry, (UIntPtr)i);
            if (namePtr == IntPtr.Zero)
            {
                continue;
            }

            result[i] = InteropUtilities.Utf8PtrToStringAndFree(namePtr) ?? string.Empty;
        }

        return result;
    }

    /// <summary>
    /// Check whether a language with the given name is available.
    /// </summary>
    public static bool HasLanguage(string name)
    {
        var namePtr = InteropUtilities.StringToUtf8Ptr(name);
        try
        {
            return NativeMethods.HasLanguage(Registry, namePtr);
        }
        finally
        {
            Marshal.FreeHGlobal(namePtr);
        }
    }

    /// <summary>
    /// Detect language name from a file path or extension.
    /// Returns null if the extension is not recognized.
    /// </summary>
    public static string? DetectLanguage(string path)
    {
        var pathPtr = InteropUtilities.StringToUtf8Ptr(path);
        try
        {
            var resultPtr = NativeMethods.DetectLanguage(pathPtr);
            if (resultPtr == IntPtr.Zero)
            {
                InteropUtilities.ThrowIfError();
                return null;
            }
            try
            {
                return InteropUtilities.Utf8PtrToString(resultPtr);
            }
            finally
            {
                NativeMethods.FreeString(resultPtr);
            }
        }
        finally
        {
            Marshal.FreeHGlobal(pathPtr);
        }
    }

    /// <summary>
    /// Detect language name from file content using shebang-based detection.
    /// Returns null if no shebang is recognized.
    /// </summary>
    public static string? DetectLanguageFromContent(string content)
    {
        var contentPtr = InteropUtilities.StringToUtf8Ptr(content);
        try
        {
            var resultPtr = NativeMethods.DetectLanguageFromContent(contentPtr);
            if (resultPtr == IntPtr.Zero)
            {
                InteropUtilities.ThrowIfError();
                return null;
            }
            try
            {
                return InteropUtilities.Utf8PtrToString(resultPtr);
            }
            finally
            {
                NativeMethods.FreeString(resultPtr);
            }
        }
        finally
        {
            Marshal.FreeHGlobal(contentPtr);
        }
    }

    /// <summary>
    /// Detect language name from a bare file extension (without leading dot).
    /// Returns null if the extension is not recognized.
    /// </summary>
    public static string? DetectLanguageFromExtension(string ext)
    {
        var extPtr = InteropUtilities.StringToUtf8Ptr(ext);
        try
        {
            var resultPtr = NativeMethods.DetectLanguageFromExtension(extPtr);
            if (resultPtr == IntPtr.Zero)
            {
                InteropUtilities.ThrowIfError();
                return null;
            }
            try
            {
                return InteropUtilities.Utf8PtrToString(resultPtr);
            }
            finally
            {
                NativeMethods.FreeString(resultPtr);
            }
        }
        finally
        {
            Marshal.FreeHGlobal(extPtr);
        }
    }

    /// <summary>
    /// Detect language name from a file path based on its extension.
    /// Returns null if the extension is not recognized.
    /// </summary>
    public static string? DetectLanguageFromPath(string path)
    {
        var pathPtr = InteropUtilities.StringToUtf8Ptr(path);
        try
        {
            var resultPtr = NativeMethods.DetectLanguageFromPath(pathPtr);
            if (resultPtr == IntPtr.Zero)
            {
                InteropUtilities.ThrowIfError();
                return null;
            }
            try
            {
                return InteropUtilities.Utf8PtrToString(resultPtr);
            }
            finally
            {
                NativeMethods.FreeString(resultPtr);
            }
        }
        finally
        {
            Marshal.FreeHGlobal(pathPtr);
        }
    }

    /// <summary>
    /// Returns ambiguity information for the given file extension.
    /// Returns null if the extension is not ambiguous.
    /// </summary>
    public static ExtensionAmbiguityResult? ExtensionAmbiguity(string ext)
    {
        var extPtr = InteropUtilities.StringToUtf8Ptr(ext);
        try
        {
            var resultPtr = NativeMethods.ExtensionAmbiguity(extPtr);
            if (resultPtr == IntPtr.Zero)
            {
                return null;
            }
            try
            {
                var json = InteropUtilities.Utf8PtrToString(resultPtr);
                if (json is null)
                {
                    return null;
                }
                return JsonSerializer.Deserialize<ExtensionAmbiguityResult>(json);
            }
            finally
            {
                NativeMethods.FreeString(resultPtr);
            }
        }
        finally
        {
            Marshal.FreeHGlobal(extPtr);
        }
    }

    /// <summary>
    /// Returns the bundled highlights query for the given language.
    /// Returns null if no bundled query is available.
    /// </summary>
    public static string? GetHighlightsQuery(string language)
    {
        var langPtr = InteropUtilities.StringToUtf8Ptr(language);
        try
        {
            var resultPtr = NativeMethods.GetHighlightsQuery(langPtr);
            if (resultPtr == IntPtr.Zero)
            {
                return null;
            }
            try
            {
                return InteropUtilities.Utf8PtrToString(resultPtr);
            }
            finally
            {
                NativeMethods.FreeString(resultPtr);
            }
        }
        finally
        {
            Marshal.FreeHGlobal(langPtr);
        }
    }

    /// <summary>
    /// Returns the bundled injections query for the given language.
    /// Returns null if no bundled query is available.
    /// </summary>
    public static string? GetInjectionsQuery(string language)
    {
        var langPtr = InteropUtilities.StringToUtf8Ptr(language);
        try
        {
            var resultPtr = NativeMethods.GetInjectionsQuery(langPtr);
            if (resultPtr == IntPtr.Zero)
            {
                return null;
            }
            try
            {
                return InteropUtilities.Utf8PtrToString(resultPtr);
            }
            finally
            {
                NativeMethods.FreeString(resultPtr);
            }
        }
        finally
        {
            Marshal.FreeHGlobal(langPtr);
        }
    }

    /// <summary>
    /// Returns the bundled locals query for the given language.
    /// Returns null if no bundled query is available.
    /// </summary>
    public static string? GetLocalsQuery(string language)
    {
        var langPtr = InteropUtilities.StringToUtf8Ptr(language);
        try
        {
            var resultPtr = NativeMethods.GetLocalsQuery(langPtr);
            if (resultPtr == IntPtr.Zero)
            {
                return null;
            }
            try
            {
                return InteropUtilities.Utf8PtrToString(resultPtr);
            }
            finally
            {
                NativeMethods.FreeString(resultPtr);
            }
        }
        finally
        {
            Marshal.FreeHGlobal(langPtr);
        }
    }

    /// <summary>
    /// Get a raw TSLanguage pointer for the given language name.
    /// </summary>
    /// <exception cref="TsPackException">Thrown when the language is not available.</exception>
    public static IntPtr GetLanguage(string name)
    {
        var namePtr = InteropUtilities.StringToUtf8Ptr(name);
        try
        {
            var result = NativeMethods.GetLanguage(Registry, namePtr);
            if (result == IntPtr.Zero)
            {
                InteropUtilities.ThrowIfError();
                throw new TsPackException($"language not found: {name}");
            }
            return result;
        }
        finally
        {
            Marshal.FreeHGlobal(namePtr);
        }
    }

    /// <summary>
    /// Get the number of available languages.
    /// </summary>
    public static int LanguageCount()
    {
        return (int)(nuint)NativeMethods.LanguageCount(Registry);
    }

    /// <summary>
    /// Parse source code with the given language and return an opaque tree handle.
    /// The caller must dispose the returned <see cref="ParseTree"/>.
    /// </summary>
    /// <exception cref="TsPackException">Thrown when parsing fails.</exception>
    public static ParseTree Parse(string languageName, string source)
    {
        var namePtr = InteropUtilities.StringToUtf8Ptr(languageName);
        var sourceBytes = Encoding.UTF8.GetBytes(source);
        var sourcePtr = Marshal.AllocHGlobal(sourceBytes.Length);
        Marshal.Copy(sourceBytes, 0, sourcePtr, sourceBytes.Length);

        try
        {
            var treePtr = NativeMethods.ParseString(
                Registry, namePtr, sourcePtr, (UIntPtr)sourceBytes.Length);

            if (treePtr == IntPtr.Zero)
            {
                var errorPtr = NativeMethods.LastError();
                var message = errorPtr != IntPtr.Zero
                    ? Marshal.PtrToStringUTF8(errorPtr) ?? "parse failed"
                    : "parse failed";
                throw new TsPackException(message);
            }

            return new ParseTree(treePtr);
        }
        finally
        {
            Marshal.FreeHGlobal(namePtr);
            Marshal.FreeHGlobal(sourcePtr);
        }
    }

    /// <summary>
    /// Process source code with the given configuration and return analysis results.
    /// </summary>
    /// <exception cref="TsPackException">Thrown when processing fails.</exception>
    public static ProcessResult Process(string source, ProcessConfig config)
    {
        var configJson = JsonSerializer.Serialize(config);
        var configPtr = InteropUtilities.StringToUtf8Ptr(configJson);
        var sourceBytes = Encoding.UTF8.GetBytes(source);
        var sourcePtr = Marshal.AllocHGlobal(sourceBytes.Length);
        Marshal.Copy(sourceBytes, 0, sourcePtr, sourceBytes.Length);

        try
        {
            var resultPtr = NativeMethods.Process(
                Registry, sourcePtr, (UIntPtr)sourceBytes.Length, configPtr);

            if (resultPtr == IntPtr.Zero)
            {
                var errorPtr = NativeMethods.LastError();
                var message = errorPtr != IntPtr.Zero
                    ? Marshal.PtrToStringUTF8(errorPtr) ?? "process failed"
                    : "process failed";
                throw new TsPackException(message);
            }

            var json = InteropUtilities.Utf8PtrToStringAndFree(resultPtr)
                ?? throw new TsPackException("null JSON result from process");

            return JsonSerializer.Deserialize<ProcessResult>(json)
                ?? throw new TsPackException("failed to deserialize process result");
        }
        finally
        {
            Marshal.FreeHGlobal(configPtr);
            Marshal.FreeHGlobal(sourcePtr);
        }
    }

    /// <summary>
    /// Initialize the language pack with configuration.
    /// configJson is a JSON string with optional fields:
    /// - "cache_dir" (string): override default cache directory
    /// - "languages" (array): languages to pre-download
    /// - "groups" (array): language groups to pre-download
    /// </summary>
    /// <exception cref="TsPackException">Thrown when initialization fails.</exception>
    public static void Init(string? configJson = null)
    {
        var configPtr = configJson != null ? InteropUtilities.StringToUtf8Ptr(configJson) : IntPtr.Zero;
        try
        {
            int rc = NativeMethods.Init(configPtr);
            if (rc != 0)
            {
                InteropUtilities.ThrowIfError();
                throw new TsPackException("ts_pack_init failed");
            }
        }
        finally
        {
            if (configPtr != IntPtr.Zero)
            {
                Marshal.FreeHGlobal(configPtr);
            }
        }
    }

    /// <summary>
    /// Configure the language pack cache directory without downloading.
    /// configJson is a JSON string with optional fields:
    /// - "cache_dir" (string): override default cache directory
    /// </summary>
    /// <exception cref="TsPackException">Thrown when configuration fails.</exception>
    public static void Configure(string? configJson = null)
    {
        var configPtr = configJson != null ? InteropUtilities.StringToUtf8Ptr(configJson) : IntPtr.Zero;
        try
        {
            int rc = NativeMethods.Configure(configPtr);
            if (rc != 0)
            {
                InteropUtilities.ThrowIfError();
                throw new TsPackException("ts_pack_configure failed");
            }
        }
        finally
        {
            if (configPtr != IntPtr.Zero)
            {
                Marshal.FreeHGlobal(configPtr);
            }
        }
    }

    /// <summary>
    /// Download specific languages to the cache.
    /// </summary>
    /// <returns>The number of newly downloaded languages.</returns>
    /// <exception cref="TsPackException">Thrown when download fails.</exception>
    public static int Download(params string[] languages)
    {
        if (languages.Length == 0)
        {
            return 0;
        }

        IntPtr[] namesPtrs = new IntPtr[languages.Length];
        try
        {
            for (int i = 0; i < languages.Length; i++)
            {
                namesPtrs[i] = InteropUtilities.StringToUtf8Ptr(languages[i]);
            }

            IntPtr namesArray = Marshal.AllocHGlobal(IntPtr.Size * languages.Length);
            for (int i = 0; i < languages.Length; i++)
            {
                Marshal.WriteIntPtr(namesArray, i * IntPtr.Size, namesPtrs[i]);
            }

            try
            {
                int rc = NativeMethods.Download(namesArray, (UIntPtr)languages.Length);
                if (rc < 0)
                {
                    InteropUtilities.ThrowIfError();
                    throw new TsPackException("ts_pack_download failed");
                }
                return rc;
            }
            finally
            {
                Marshal.FreeHGlobal(namesArray);
            }
        }
        finally
        {
            for (int i = 0; i < namesPtrs.Length; i++)
            {
                if (namesPtrs[i] != IntPtr.Zero)
                {
                    Marshal.FreeHGlobal(namesPtrs[i]);
                }
            }
        }
    }

    /// <summary>
    /// Download all available languages from the remote manifest.
    /// </summary>
    /// <returns>The number of newly downloaded languages.</returns>
    /// <exception cref="TsPackException">Thrown when download fails.</exception>
    public static int DownloadAll()
    {
        int rc = NativeMethods.DownloadAll();
        if (rc < 0)
        {
            InteropUtilities.ThrowIfError();
            throw new TsPackException("ts_pack_download_all failed");
        }
        return rc;
    }

    /// <summary>
    /// Get all language names available in the remote manifest.
    /// </summary>
    /// <returns>An array of available language names.</returns>
    /// <exception cref="TsPackException">Thrown when the operation fails.</exception>
    public static string[] ManifestLanguages()
    {
        var arrPtr = NativeMethods.ManifestLanguages(out var count);
        if (arrPtr == IntPtr.Zero)
        {
            InteropUtilities.ThrowIfError();
            throw new TsPackException("ts_pack_manifest_languages failed");
        }

        try
        {
            var result = new string[(int)(nuint)count];
            for (int i = 0; i < (int)(nuint)count; i++)
            {
                IntPtr strPtr = Marshal.ReadIntPtr(arrPtr, i * IntPtr.Size);
                result[i] = Marshal.PtrToStringUTF8(strPtr) ?? string.Empty;
                // Free each individual string before freeing the array
                NativeMethods.FreeString(strPtr);
            }
            return result;
        }
        finally
        {
            NativeMethods.FreeStringArray(arrPtr);
        }
    }

    /// <summary>
    /// Get all languages that are already downloaded and cached locally.
    /// </summary>
    /// <returns>An array of locally cached language names.</returns>
    /// <exception cref="TsPackException">Thrown when the operation fails.</exception>
    public static string[] DownloadedLanguages()
    {
        var arrPtr = NativeMethods.DownloadedLanguages(out var count);
        if (arrPtr == IntPtr.Zero)
        {
            return [];
        }

        try
        {
            var result = new string[(int)(nuint)count];
            for (int i = 0; i < (int)(nuint)count; i++)
            {
                IntPtr strPtr = Marshal.ReadIntPtr(arrPtr, i * IntPtr.Size);
                result[i] = Marshal.PtrToStringUTF8(strPtr) ?? string.Empty;
                // Free each individual string before freeing the array
                NativeMethods.FreeString(strPtr);
            }
            return result;
        }
        finally
        {
            NativeMethods.FreeStringArray(arrPtr);
        }
    }

    /// <summary>
    /// Delete all cached parser shared libraries.
    /// </summary>
    /// <exception cref="TsPackException">Thrown when the operation fails.</exception>
    public static void CleanCache()
    {
        int rc = NativeMethods.CleanCache();
        if (rc != 0)
        {
            InteropUtilities.ThrowIfError();
            throw new TsPackException("ts_pack_clean_cache failed");
        }
    }

    /// <summary>
    /// Get the effective cache directory path.
    /// </summary>
    /// <returns>The cache directory path as a string.</returns>
    /// <exception cref="TsPackException">Thrown when the operation fails.</exception>
    public static string? CacheDir()
    {
        var cachePtr = NativeMethods.CacheDir();
        if (cachePtr == IntPtr.Zero)
        {
            InteropUtilities.ThrowIfError();
            throw new TsPackException("ts_pack_cache_dir failed");
        }
        return InteropUtilities.Utf8PtrToStringAndFree(cachePtr);
    }
}

/// <summary>
/// An opaque handle to a parsed syntax tree. Must be disposed to free native memory.
/// </summary>
public sealed class ParseTree : IDisposable
{
    private IntPtr _handle;
    private bool _disposed;

    internal ParseTree(IntPtr handle)
    {
        _handle = handle;
    }

    /// <summary>
    /// Get the type name of the root node.
    /// </summary>
    public string? RootNodeType()
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        return InteropUtilities.Utf8PtrToStringAndFree(NativeMethods.TreeRootNodeType(_handle));
    }

    /// <summary>
    /// Get the number of named children of the root node.
    /// </summary>
    public uint RootChildCount()
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        return NativeMethods.TreeRootChildCount(_handle);
    }

    /// <summary>
    /// Check whether the tree contains a node with the given type name.
    /// </summary>
    public bool ContainsNodeType(string nodeType)
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        var ptr = InteropUtilities.StringToUtf8Ptr(nodeType);
        try
        {
            return NativeMethods.TreeContainsNodeType(_handle, ptr);
        }
        finally
        {
            Marshal.FreeHGlobal(ptr);
        }
    }

    /// <summary>
    /// Check whether the tree contains any ERROR or MISSING nodes.
    /// </summary>
    public bool HasErrorNodes()
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        return NativeMethods.TreeHasErrorNodes(_handle);
    }

    /// <summary>
    /// Return the S-expression representation of the tree.
    /// </summary>
    public string? ToSexp()
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        return InteropUtilities.Utf8PtrToStringAndFree(NativeMethods.TreeToSexp(_handle));
    }

    /// <summary>
    /// Return the count of ERROR and MISSING nodes in the tree.
    /// </summary>
    public int ErrorCount()
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        return (int)(nuint)NativeMethods.TreeErrorCount(_handle);
    }

    /// <inheritdoc/>
    public void Dispose()
    {
        if (!_disposed)
        {
            if (_handle != IntPtr.Zero)
            {
                NativeMethods.TreeFree(_handle);
                _handle = IntPtr.Zero;
            }
            _disposed = true;
        }
    }
}
