---
id: fixture_csharp_smoke_hcl
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("variable \"name\" { type = string }", new ProcessConfig { Language = "hcl" });

```
