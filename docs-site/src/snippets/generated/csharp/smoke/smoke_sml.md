---
id: fixture_csharp_smoke_sml
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("x", new ProcessConfig { Language = "sml" });

```
