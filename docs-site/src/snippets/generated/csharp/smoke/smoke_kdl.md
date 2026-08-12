---
id: fixture_csharp_smoke_kdl
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("node \"value\"", new ProcessConfig { Language = "kdl" });

```
