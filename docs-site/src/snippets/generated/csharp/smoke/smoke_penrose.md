---
id: fixture_csharp_smoke_penrose
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("type Set\n", new ProcessConfig { Language = "penrose" });

```
