---
id: fixture_csharp_smoke_clarity
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("(define-public (hello) (ok true))", new ProcessConfig { Language = "clarity" });

```
