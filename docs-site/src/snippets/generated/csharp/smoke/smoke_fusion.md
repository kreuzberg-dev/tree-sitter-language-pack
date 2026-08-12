---
id: fixture_csharp_smoke_fusion
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("foo = 1\n", new ProcessConfig { Language = "fusion" });

```
