---
id: fixture_swift_smoke_turtle
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"turtle\"}")
_ = try TreeSitterLanguagePack.process(source: "x", config: configObj)

```
