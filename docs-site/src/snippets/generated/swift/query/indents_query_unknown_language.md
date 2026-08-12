---
id: fixture_swift_indents_query_unknown_language
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

_ = try TreeSitterLanguagePack.getIndentsQuery(language: "nonexistent_xyz")

```
