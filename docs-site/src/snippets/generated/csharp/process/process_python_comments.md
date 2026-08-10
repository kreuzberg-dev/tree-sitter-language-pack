```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("# This is a comment\n# Another comment\ndef hello():\n    # inline comment\n    pass\n", new ProcessConfig { Comments = true, Language = "python" });

```
