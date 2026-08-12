---
id: fixture_csharp_smoke_pkl
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("name = \"hello\"", new ProcessConfig { Language = "pkl" });

```
