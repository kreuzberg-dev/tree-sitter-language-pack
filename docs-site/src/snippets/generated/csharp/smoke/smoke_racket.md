```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("#lang racket\n(define x 1)", new ProcessConfig { Language = "racket" });

```
