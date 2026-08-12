---
id: fixture_csharp_smoke_d2
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("a -> b\n", new ProcessConfig { Language = "d2" });

```
