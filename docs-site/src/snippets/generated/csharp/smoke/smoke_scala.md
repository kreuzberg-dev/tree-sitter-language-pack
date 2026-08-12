---
id: fixture_csharp_smoke_scala
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("object Main", new ProcessConfig { Language = "scala" });

```
