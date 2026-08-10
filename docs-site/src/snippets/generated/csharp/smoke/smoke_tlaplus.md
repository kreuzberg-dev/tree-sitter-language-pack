```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("---- MODULE Main ----\n====", new ProcessConfig { Language = "tlaplus" });

```
