---
id: fixture_csharp_smoke_rego
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("package main\ndefault allow = false", new ProcessConfig { Language = "rego" });

```
