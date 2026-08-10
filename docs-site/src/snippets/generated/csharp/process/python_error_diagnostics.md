```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("def broken(\n    pass\n", new ProcessConfig { Diagnostics = true, Language = "python" });

```
