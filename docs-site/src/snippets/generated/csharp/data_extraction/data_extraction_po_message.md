```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("msgid \"Hello\"\nmsgstr \"Hallo\"\n", new ProcessConfig { DataExtraction = true, Language = "po" });

```
