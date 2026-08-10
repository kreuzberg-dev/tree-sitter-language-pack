```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("data _null_;\nrun;\n", new ProcessConfig { Language = "sas" });

```
