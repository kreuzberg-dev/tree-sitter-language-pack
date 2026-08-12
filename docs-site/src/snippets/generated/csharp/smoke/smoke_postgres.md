---
id: fixture_csharp_smoke_postgres
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("SELECT 1;\n", new ProcessConfig { Language = "postgres" });

```
