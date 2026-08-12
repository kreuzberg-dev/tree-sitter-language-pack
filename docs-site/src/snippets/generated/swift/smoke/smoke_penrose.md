---
id: fixture_swift_smoke_penrose
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"penrose\"}")
_ = try TreeSitterLanguagePack.process(source: "type Set\n", config: configObj)

```
