---
id: fixture_csharp_smoke_lalrpop
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("grammar;\n", new ProcessConfig { Language = "lalrpop" });

```
