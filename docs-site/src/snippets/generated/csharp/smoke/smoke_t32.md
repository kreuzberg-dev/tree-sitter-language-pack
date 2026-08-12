---
id: fixture_csharp_smoke_t32
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("PRINT 1\n", new ProcessConfig { Language = "t32" });

```
