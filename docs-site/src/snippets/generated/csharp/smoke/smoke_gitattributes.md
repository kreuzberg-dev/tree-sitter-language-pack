---
id: fixture_csharp_smoke_gitattributes
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("*.txt text", new ProcessConfig { Language = "gitattributes" });

```
