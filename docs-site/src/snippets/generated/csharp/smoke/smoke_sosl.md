---
id: fixture_csharp_smoke_sosl
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("FIND {test}\n", new ProcessConfig { Language = "sosl" });

```
