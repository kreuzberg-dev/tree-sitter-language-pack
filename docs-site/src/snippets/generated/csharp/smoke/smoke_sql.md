```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("SELECT 1;", new ProcessConfig { Language = "sql" });

```
