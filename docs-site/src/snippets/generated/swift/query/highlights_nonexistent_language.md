---
id: fixture_swift_highlights_nonexistent_language
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

_ = try TreeSitterLanguagePack.getHighlightsQuery(language: "zzz_nonexistent_lang")

```
