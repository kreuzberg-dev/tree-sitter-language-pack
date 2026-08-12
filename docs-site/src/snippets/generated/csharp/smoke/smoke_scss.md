---
id: fixture_csharp_smoke_scss
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("$color: red;\nbody { color: $color; }", new ProcessConfig { Language = "scss" });

```
