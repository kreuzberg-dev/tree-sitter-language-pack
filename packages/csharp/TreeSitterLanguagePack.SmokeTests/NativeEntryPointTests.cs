using System.Runtime.InteropServices;
using TreeSitterLanguagePack;
using Xunit;

namespace TreeSitterLanguagePack.SmokeTests;

/// <summary>
/// Touches every P/Invoke entry point that a native library predating the
/// v1.12.0 indents/folds/prefetch additions does not export.
///
/// .NET resolves <c>DllImport</c> entry points lazily, on first call, so a
/// stale <c>libts_pack_core_ffi</c> produces a package that builds, packs,
/// publishes, and then throws <see cref="EntryPointNotFoundException"/> in a
/// consumer's production process. These tests move that failure to CI.
/// </summary>
public sealed class NativeEntryPointTests {
  // ~keep: these five names are the C# half of the audited missing-symbol set.
  // Removing a case here reopens the exact gap this file was added to close;
  // add a case whenever a new entry point lands in NativeMethods.cs.
  public static TheoryData<string, Action> EntryPoints => new() {
    { "ts_pack_get_indents_query",
      () => TreeSitterLanguagePackConverter.GetIndentsQuery("python") },
    { "ts_pack_get_folds_query",
      () => TreeSitterLanguagePackConverter.GetFoldsQuery("python") },
    { "ts_pack_prefetch",
      () => TreeSitterLanguagePackConverter.Prefetch(new List<string>()) },
    { "ts_pack_parser_default", () => Parser.Default() },
    { "ts_pack_language_registry_default", () => LanguageRegistry.Default() },
  };

  [Theory]
  [MemberData(nameof(EntryPoints))]
  public void ResolvesEntryPointAgainstLoadedNativeLibrary(string entryPoint,
                                                           Action invoke) {
    var exception = Record.Exception(invoke);

    Assert.False(
        exception is EntryPointNotFoundException,
        $"The loaded native library does not export {entryPoint}. It is stale " +
            "relative to crates/ts-pack-core-ffi/include/ts_pack.h — rebuild " +
            "ts-pack-core-ffi and refresh " +
            "packages/csharp/TreeSitterLanguagePack/runtimes/<rid>/native/.");

    // A functional failure (missing grammar, no network) is out of scope here;
    // only the unresolvable-symbol case indicates a stale library.
    Assert.False(
        exception is DllNotFoundException,
        $"ts_pack_core_ffi could not be loaded at all while resolving " +
            $"{entryPoint}. Put the shared library on the loader " +
            "path (DYLD_LIBRARY_PATH / LD_LIBRARY_PATH / PATH).");
  }

  [Fact]
  public void LoadsNativeLibraryAndReportsLanguages() {
    Assert.True(TreeSitterLanguagePackConverter.LanguageCount() > 0,
                "LanguageCount() returned 0; the native library loaded but " +
                    "reports no languages.");
  }
}
