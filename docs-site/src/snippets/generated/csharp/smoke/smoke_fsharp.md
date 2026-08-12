---
id: fixture_csharp_smoke_fsharp
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("let x = 1", new ProcessConfig { Language = "fsharp" });

```
