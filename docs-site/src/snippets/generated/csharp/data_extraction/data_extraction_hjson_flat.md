```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("{\n  host: \"localhost\"\n  port: 8080\n}\n", new ProcessConfig { DataExtraction = true, Language = "hjson" });

```
