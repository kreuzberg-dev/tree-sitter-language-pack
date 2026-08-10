```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("const x = 1;", new ProcessConfig { Language = "javascript" });

```
