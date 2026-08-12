---
id: fixture_csharp_smoke_bibtex
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("@article{key, title={A}}", new ProcessConfig { Language = "bibtex" });

```
