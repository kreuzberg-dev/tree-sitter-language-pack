```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("msgid \"hello\"\nmsgstr \"world\"", new ProcessConfig { Language = "po" });

```
