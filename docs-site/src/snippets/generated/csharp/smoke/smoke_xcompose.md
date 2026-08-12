---
id: fixture_csharp_smoke_xcompose
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("<Multi_key> <a> : \"a\"", new ProcessConfig { Language = "xcompose" });

```
