---
id: fixture_swift_smoke_elvish
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"elvish\"}")
_ = try TreeSitterLanguagePack.process(source: "x", config: configObj)

```
