```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("{\"server\": {\"host\": \"x\", \"port\": 8080}}", new ProcessConfig { DataExtraction = true, Language = "json" });

```
