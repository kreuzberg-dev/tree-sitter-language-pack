---
id: fixture_csharp_smoke_sas
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("data _null_;\nrun;\n", new ProcessConfig { Language = "sas" });

```
