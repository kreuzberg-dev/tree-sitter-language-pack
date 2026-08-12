---
id: fixture_csharp_smoke_ninja
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("rule cc\n  command = cc $in -o $out", new ProcessConfig { Language = "ninja" });

```
