---
id: fixture_csharp_smoke_koto
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("x = 1\n", new ProcessConfig { Language = "koto" });

```
