```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("x,y,z\n", new ProcessConfig { DataExtraction = true, Language = "csv" });

```
