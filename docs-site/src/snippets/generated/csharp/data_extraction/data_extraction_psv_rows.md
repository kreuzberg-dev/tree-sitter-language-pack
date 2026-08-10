```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("a|b|c\n1|2|3\n", new ProcessConfig { DataExtraction = true, Language = "psv" });

```
