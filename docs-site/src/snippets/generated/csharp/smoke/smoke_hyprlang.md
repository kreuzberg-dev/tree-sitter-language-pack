---
id: fixture_csharp_smoke_hyprlang
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("general { border_size = 1 }", new ProcessConfig { Language = "hyprlang" });

```
