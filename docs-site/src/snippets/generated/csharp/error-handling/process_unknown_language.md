```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("x = 1", new ProcessConfig { Language = "nonexistent_xyz" });

```
