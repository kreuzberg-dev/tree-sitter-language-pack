---
id: fixture_csharp_smoke_edoc
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("@doc foo\n", new ProcessConfig { Language = "edoc" });

```
