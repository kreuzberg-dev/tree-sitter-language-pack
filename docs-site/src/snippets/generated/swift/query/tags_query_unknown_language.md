---
id: fixture_swift_tags_query_unknown_language
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

_ = try TreeSitterLanguagePack.getTagsQuery(language: "nonexistent_xyz")

```
