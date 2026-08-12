---
id: fixture_swift_smoke_scala
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"scala\"}")
_ = try TreeSitterLanguagePack.process(source: "object Main", config: configObj)

```
