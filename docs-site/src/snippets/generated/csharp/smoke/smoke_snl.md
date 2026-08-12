---
id: fixture_csharp_smoke_snl
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("program test\n", new ProcessConfig { Language = "snl" });

```
