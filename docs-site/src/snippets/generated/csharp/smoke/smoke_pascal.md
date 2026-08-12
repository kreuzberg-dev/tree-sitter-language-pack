---
id: fixture_csharp_smoke_pascal
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("program Hello; begin end.", new ProcessConfig { Language = "pascal" });

```
