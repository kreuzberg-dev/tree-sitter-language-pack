---
id: fixture_csharp_smoke_luap
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("[a-z]+", new ProcessConfig { Language = "luap" });

```
