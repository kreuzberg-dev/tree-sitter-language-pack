---
id: fixture_swift_locals_query_unknown_language
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

_ = try TreeSitterLanguagePack.getLocalsQuery(language: "nonexistent_xyz")

```
