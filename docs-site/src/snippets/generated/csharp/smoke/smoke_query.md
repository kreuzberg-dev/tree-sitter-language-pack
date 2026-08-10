```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("(identifier) @name", new ProcessConfig { Language = "query" });

```
