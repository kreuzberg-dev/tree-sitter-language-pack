---
id: fixture_csharp_smoke_csv
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("a,b,c\n1,2,3", new ProcessConfig { Language = "csv" });

```
