---
id: fixture_swift_smoke_query
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"query\"}")
_ = try TreeSitterLanguagePack.process(source: "(identifier) @name", config: configObj)

```
