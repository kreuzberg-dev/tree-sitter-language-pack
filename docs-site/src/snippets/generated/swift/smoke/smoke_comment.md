---
id: fixture_swift_smoke_comment
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"comment\"}")
_ = try TreeSitterLanguagePack.process(source: "Review: handle edge case", config: configObj)

```
