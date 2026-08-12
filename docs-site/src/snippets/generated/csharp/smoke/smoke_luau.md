---
id: fixture_csharp_smoke_luau
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("local x: number = 1", new ProcessConfig { Language = "luau" });

```
