---
id: fixture_csharp_smoke_printf
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("%d %s", new ProcessConfig { Language = "printf" });

```
