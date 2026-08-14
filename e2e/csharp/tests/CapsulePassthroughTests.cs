// Hand-written capsule-passthrough test (not fixture-generated); preserved
// across regen.

using System;
using Xunit;
using TreeSitterLanguagePack;

namespace TreeSitterLanguagePack {
/// <summary>
/// Verify that TreeSitterLanguagePackConverter.GetLanguage returns a
/// host-native TreeSitter.Language usable with the upstream TreeSitter.DotNet
/// parser.
///
/// This tests the capsule passthrough feature (#143): the host-native Language
/// wrapper allows direct interop with third-party tree-sitter consumers.
/// </summary>
public class CapsulePassthroughTests {
  [Fact]
  public void Test_ParsePythonWithHostLanguage() {
    // Get host-native Language via TreeSitterLanguagePackConverter
    var language = TreeSitterLanguagePackConverter.GetLanguage("python");
    Assert.NotNull(language);

    // Create upstream TreeSitter.DotNet parser
    using var parser = new TreeSitter.Parser();

    // Set the host-native language on the parser
    parser.Language = language;

    // Parse Python source
    var source = "def greet(name):\n    return name\n";
    using var tree = parser.Parse(source);

    var rootNode = tree.RootNode;
    Assert.NotNull(rootNode);

    // Verify the root node type is "module" (Python AST root)
    Assert.Equal("module", rootNode.Type);
  }

  [Fact]
  public void Test_ParseJavascriptWithHostLanguage() {
    // Get host-native Language via TreeSitterLanguagePackConverter
    var language = TreeSitterLanguagePackConverter.GetLanguage("javascript");
    Assert.NotNull(language);

    // Create upstream TreeSitter.DotNet parser
    using var parser = new TreeSitter.Parser();

    // Set the host-native language on the parser
    parser.Language = language;

    // Parse JavaScript source
    var source = "const x = 1;\n";
    using var tree = parser.Parse(source);

    var rootNode = tree.RootNode;
    Assert.NotNull(rootNode);

    // Verify the root node type is "program" (JavaScript AST root)
    Assert.Equal("program", rootNode.Type);
  }

  /// <summary>
  /// The capsule contract declared in alef.toml claims the pointer is
  /// borrowed-static and that TreeSitter.DotNet's destructor is an ABI-level
  /// no-op. Every GetLanguage call wraps the SAME static pointer in a NEW
  /// TreeSitter.Language, so disposing one wrapper and then parsing through
  /// another is the case that breaks if ts_language_delete ever really frees.
  /// </summary>
  [Fact]
  public void Test_LanguagePointerSurvivesHostDisposeAndFinalization() {
    // Read the ABI version through the OTHER runtime — this dereferences our
    // TSLanguage using TreeSitter.DotNet's own struct layout.
    var probe = TreeSitterLanguagePackConverter.GetLanguage("python");
    Assert.Equal(14u, probe.AbiVersion);

    // Explicit dispose, then double-dispose, on a wrapper aliasing the pointer.
    probe.Dispose();
    probe.Dispose();

    // Drop every managed reference and force the finalizer path to run, which
    // is where TreeSitter.DotNet calls ts_language_delete unprompted.
    for (var i = 0; i < 16; i++) {
      TreeSitterLanguagePackConverter.GetLanguage("python");
    }
    GC.Collect();
    GC.WaitForPendingFinalizers();
    GC.Collect();

    // The pointer must still be live and parseable after all of the above.
    var revived = TreeSitterLanguagePackConverter.GetLanguage("python");
    Assert.Equal(14u, revived.AbiVersion);

    using var parser = new TreeSitter.Parser();
    parser.Language = revived;
    using var tree = parser.Parse("def greet(name):\n    return name\n");
    Assert.Equal("module", tree.RootNode.Type);
  }
}
}
