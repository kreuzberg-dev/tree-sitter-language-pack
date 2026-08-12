---
id: fixture_csharp_highlights_query_unknown_language
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.GetHighlightsQuery("nonexistent_language_xyz");

```
