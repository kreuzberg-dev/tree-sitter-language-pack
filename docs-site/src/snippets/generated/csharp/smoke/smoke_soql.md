```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("SELECT Id FROM Account\n", new ProcessConfig { Language = "soql" });

```
