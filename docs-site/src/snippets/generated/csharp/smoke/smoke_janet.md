---
id: fixture_csharp_smoke_janet
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("(print \"hello\")", new ProcessConfig { Language = "janet" });

```
