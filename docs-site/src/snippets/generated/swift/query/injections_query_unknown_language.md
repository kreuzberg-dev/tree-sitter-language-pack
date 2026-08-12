---
id: fixture_swift_injections_query_unknown_language
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

_ = try TreeSitterLanguagePack.getInjectionsQuery(language: "nonexistent_xyz")

```
