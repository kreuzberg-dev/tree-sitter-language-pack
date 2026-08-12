---
id: fixture_csharp_smoke_fluent
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("hello = Hello\n", new ProcessConfig { Language = "fluent" });

```
