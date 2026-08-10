```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("let x = 1;\n", new ProcessConfig { Language = "reason" });

```
