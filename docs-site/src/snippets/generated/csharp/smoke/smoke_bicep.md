---
id: fixture_csharp_smoke_bicep
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("param name string", new ProcessConfig { Language = "bicep" });

```
