---
id: fixture_csharp_highlights_nonexistent_language
language: csharp
target: csharp
level: typecheck
requires: []
side_effect: safe
---

```csharp title="C#"
using TreeSitterLanguagePack;

var result = TreeSitterLanguagePackConverter.GetHighlightsQuery("zzz_nonexistent_lang");

```
