---
id: fixture_csharp_tags_query_unknown_language
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.GetTagsQuery("nonexistent_xyz");

```
