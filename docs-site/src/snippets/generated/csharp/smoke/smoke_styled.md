---
id: fixture_csharp_smoke_styled
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("color: red;\n", new ProcessConfig { Language = "styled" });

```
