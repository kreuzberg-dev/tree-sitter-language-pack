---
id: fixture_csharp_smoke_query
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("(identifier) @name", new ProcessConfig { Language = "query" });

```
