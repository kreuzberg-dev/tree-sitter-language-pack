---
id: fixture_csharp_smoke_requirements
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("flask>=2.0", new ProcessConfig { Language = "requirements" });

```
