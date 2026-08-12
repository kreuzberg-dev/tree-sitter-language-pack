---
id: fixture_csharp_smoke_picat
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("main => true.\n", new ProcessConfig { Language = "picat" });

```
