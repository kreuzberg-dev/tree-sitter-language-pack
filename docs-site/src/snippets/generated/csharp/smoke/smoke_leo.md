---
id: fixture_csharp_smoke_leo
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("program test.aleo {\n}\n", new ProcessConfig { Language = "leo" });

```
