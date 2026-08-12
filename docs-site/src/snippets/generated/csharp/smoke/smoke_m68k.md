---
id: fixture_csharp_smoke_m68k
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process(" move.l d0,d1\n", new ProcessConfig { Language = "m68k" });

```
