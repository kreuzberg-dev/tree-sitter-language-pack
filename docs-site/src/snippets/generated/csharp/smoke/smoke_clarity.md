```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("(define-public (hello) (ok true))", new ProcessConfig { Language = "clarity" });

```
