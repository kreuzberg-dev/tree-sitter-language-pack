```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("region = \"us-east-1\"\ncount  = 3\n", new ProcessConfig { DataExtraction = true, Language = "hcl" });

```
