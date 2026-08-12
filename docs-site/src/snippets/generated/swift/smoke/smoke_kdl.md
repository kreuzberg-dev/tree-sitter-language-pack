---
id: fixture_swift_smoke_kdl
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"kdl\"}")
_ = try TreeSitterLanguagePack.process(source: "node \"value\"", config: configObj)

```
