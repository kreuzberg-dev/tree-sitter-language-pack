---
id: fixture_csharp_smoke_cedarschema
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("entity User;", new ProcessConfig { Language = "cedarschema" });

```
