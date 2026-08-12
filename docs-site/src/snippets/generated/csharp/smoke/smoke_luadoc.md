---
id: fixture_csharp_smoke_luadoc
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("---@param name string", new ProcessConfig { Language = "luadoc" });

```
