---
id: fixture_csharp_smoke_fennel
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("(fn hello [] (print :hello))", new ProcessConfig { Language = "fennel" });

```
