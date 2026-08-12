---
id: fixture_swift_highlights_query_unknown_language
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

_ = try TreeSitterLanguagePack.getHighlightsQuery(language: "nonexistent_language_xyz")

```
