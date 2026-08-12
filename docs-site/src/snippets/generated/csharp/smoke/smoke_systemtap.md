---
id: fixture_csharp_smoke_systemtap
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("probe begin {}\n", new ProcessConfig { Language = "systemtap" });

```
