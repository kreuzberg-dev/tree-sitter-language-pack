---
id: fixture_csharp_smoke_vhdl
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("entity main is end main;", new ProcessConfig { Language = "vhdl" });

```
