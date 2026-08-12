---
id: fixture_csharp_smoke_wit
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("package example:pkg;", new ProcessConfig { Language = "wit" });

```
