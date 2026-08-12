---
id: fixture_swift_smoke_bicep
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"bicep\"}")
_ = try TreeSitterLanguagePack.process(source: "param name string", config: configObj)

```
