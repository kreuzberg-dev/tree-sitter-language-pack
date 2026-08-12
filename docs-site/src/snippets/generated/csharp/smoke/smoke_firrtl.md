---
id: fixture_csharp_smoke_firrtl
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("circuit Main :", new ProcessConfig { Language = "firrtl" });

```
