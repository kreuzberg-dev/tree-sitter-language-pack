---
id: fixture_swift_smoke_firrtl
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"firrtl\"}")
_ = try TreeSitterLanguagePack.process(source: "circuit Main :", config: configObj)

```
