---
id: fixture_csharp_smoke_gren
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("module Main exposing (..)", new ProcessConfig { Language = "gren" });

```
