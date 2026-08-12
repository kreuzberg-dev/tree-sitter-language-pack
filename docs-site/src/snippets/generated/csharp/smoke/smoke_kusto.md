---
id: fixture_csharp_smoke_kusto
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("T | count\n", new ProcessConfig { Language = "kusto" });

```
