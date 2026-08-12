---
id: fixture_csharp_smoke_gcode
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("G0 X0\n", new ProcessConfig { Language = "gcode" });

```
