---
id: fixture_csharp_smoke_commonlisp
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("(defun hello () (print \"hello\"))", new ProcessConfig { Language = "commonlisp" });

```
