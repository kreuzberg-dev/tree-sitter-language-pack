```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("server:\n  host: localhost\n  port: 8080\n", new ProcessConfig { DataExtraction = true, Language = "yaml" });

```
