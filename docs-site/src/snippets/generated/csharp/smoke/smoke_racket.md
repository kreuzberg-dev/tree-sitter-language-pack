---
id: fixture_csharp_smoke_racket
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("#lang racket\n(define x 1)", new ProcessConfig { Language = "racket" });

```
