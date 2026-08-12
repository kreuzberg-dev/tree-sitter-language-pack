---
id: fixture_swift_smoke_teal
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"teal\"}")
_ = try TreeSitterLanguagePack.process(source: "x", config: configObj)

```
