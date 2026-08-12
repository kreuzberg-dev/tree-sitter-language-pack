---
id: fixture_csharp_smoke_agda
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("module Main where", new ProcessConfig { Language = "agda" });

```
