---
id: fixture_csharp_smoke_sql
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("SELECT 1;", new ProcessConfig { Language = "sql" });

```
