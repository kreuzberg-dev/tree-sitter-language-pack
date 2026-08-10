```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("[server]\nhost = \"localhost\"\nport = 8080\n", new ProcessConfig { DataExtraction = true, Language = "toml" });

```
