```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("       IDENTIFICATION DIVISION.\n       PROGRAM-ID. HELLO.", new ProcessConfig { Language = "cobol" });

```
