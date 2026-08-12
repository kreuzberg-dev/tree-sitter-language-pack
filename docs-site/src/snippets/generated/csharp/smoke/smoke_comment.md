---
id: fixture_csharp_smoke_comment
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.Process("Review: handle edge case", new ProcessConfig { Language = "comment" });

```
