```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("server.host=localhost\nserver.port=8080\n", new ProcessConfig { DataExtraction = true, Language = "properties" });

```
