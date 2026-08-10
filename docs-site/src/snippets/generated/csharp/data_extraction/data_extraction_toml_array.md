```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("ports = [8080, 8081, 8082]\n", new ProcessConfig { DataExtraction = true, Language = "toml" });

```
