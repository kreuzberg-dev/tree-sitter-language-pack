```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("{\"host\": \"localhost\", \"port\": 8080}", new ProcessConfig { DataExtraction = true, Language = "json" });

```
