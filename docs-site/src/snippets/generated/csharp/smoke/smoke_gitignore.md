```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("*.o\n*.log", new ProcessConfig { Language = "gitignore" });

```
