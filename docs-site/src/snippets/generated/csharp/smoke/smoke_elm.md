---
id: fixture_csharp_smoke_elm
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("module Main exposing (..)", new ProcessConfig { Language = "elm" });

```
