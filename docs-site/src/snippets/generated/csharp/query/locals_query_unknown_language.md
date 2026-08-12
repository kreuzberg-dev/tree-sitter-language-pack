---
id: fixture_csharp_locals_query_unknown_language
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.GetLocalsQuery("nonexistent_xyz");

```
