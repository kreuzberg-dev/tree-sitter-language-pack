---
id: fixture_csharp_smoke_idris
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("module Main", new ProcessConfig { Language = "idris" });

```
