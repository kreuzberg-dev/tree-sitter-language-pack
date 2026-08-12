---
id: fixture_csharp_smoke_sxhkdrc
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("super + a\n\techo hi\n", new ProcessConfig { Language = "sxhkdrc" });

```
