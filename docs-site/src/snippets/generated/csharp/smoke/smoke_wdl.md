---
id: fixture_csharp_smoke_wdl
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("version 1.0\n", new ProcessConfig { Language = "wdl" });

```
