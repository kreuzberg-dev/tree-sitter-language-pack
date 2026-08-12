---
id: fixture_csharp_smoke_gosum
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("example.com/pkg v1.0.0 h1:abc=", new ProcessConfig { Language = "gosum" });

```
