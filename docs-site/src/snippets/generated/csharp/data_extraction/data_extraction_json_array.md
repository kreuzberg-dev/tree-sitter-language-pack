```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("[1, 2, 3]", new ProcessConfig { DataExtraction = true, Language = "json" });

```
