---
id: fixture_csharp_smoke_soql
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("SELECT Id FROM Account\n", new ProcessConfig { Language = "soql" });

```
