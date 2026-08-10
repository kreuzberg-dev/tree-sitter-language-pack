```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("(define x 1)", new ProcessConfig { Language = "scheme" });

```
