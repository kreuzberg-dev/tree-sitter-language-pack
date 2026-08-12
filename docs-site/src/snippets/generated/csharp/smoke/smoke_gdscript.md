---
id: fixture_csharp_smoke_gdscript
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("extends Node\nfunc _ready():\n\tpass", new ProcessConfig { Language = "gdscript" });

```
