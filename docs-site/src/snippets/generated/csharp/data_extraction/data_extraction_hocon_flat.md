```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("host = \"localhost\"\nport = 8080\n", new ProcessConfig { DataExtraction = true, Language = "hocon" });

```
