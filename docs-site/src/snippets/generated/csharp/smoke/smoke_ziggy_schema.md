---
id: fixture_csharp_smoke_ziggy_schema
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("x", new ProcessConfig { Language = "ziggy_schema" });

```
