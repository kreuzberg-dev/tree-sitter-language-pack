---
id: fixture_csharp_injections_query_unknown_language
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.GetInjectionsQuery("nonexistent_xyz");

```
