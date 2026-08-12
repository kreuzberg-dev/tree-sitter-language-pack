---
id: fixture_csharp_smoke_rescript
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("let x = 1", new ProcessConfig { Language = "rescript" });

```
