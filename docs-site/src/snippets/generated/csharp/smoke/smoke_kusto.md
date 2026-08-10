```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("T | count\n", new ProcessConfig { Language = "kusto" });

```
