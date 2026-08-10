```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("<server id=\"main\"><host>localhost</host></server>", new ProcessConfig { DataExtraction = true, Language = "xml" });

```
