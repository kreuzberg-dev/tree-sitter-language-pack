---
id: fixture_csharp_smoke_menhir
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("%token EOF\n%%\n", new ProcessConfig { Language = "menhir" });

```
