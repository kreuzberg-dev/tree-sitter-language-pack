---
id: fixture_csharp_smoke_starlark
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("def hello(): pass", new ProcessConfig { Language = "starlark" });

```
