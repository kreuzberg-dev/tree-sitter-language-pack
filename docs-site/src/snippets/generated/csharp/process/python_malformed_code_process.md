```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("def broken(\n    return\nclass", new ProcessConfig { Diagnostics = true, Language = "python" });

```
