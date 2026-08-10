```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("ports:\n  - 8080\n  - 8081\n", new ProcessConfig { DataExtraction = true, Language = "yaml" });

```
