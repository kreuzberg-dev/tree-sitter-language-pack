---
id: fixture_csharp_smoke_tlaplus
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("---- MODULE Main ----\n====", new ProcessConfig { Language = "tlaplus" });

```
