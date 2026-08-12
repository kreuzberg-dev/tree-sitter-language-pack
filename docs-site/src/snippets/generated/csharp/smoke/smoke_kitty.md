---
id: fixture_csharp_smoke_kitty
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("font_size 12\n", new ProcessConfig { Language = "kitty" });

```
