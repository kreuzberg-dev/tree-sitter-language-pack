---
id: fixture_csharp_smoke_jsdoc
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("/** @param {string} name */", new ProcessConfig { Language = "jsdoc" });

```
