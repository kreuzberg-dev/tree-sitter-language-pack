---
id: fixture_swift_smoke_sosl
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"sosl\"}")
_ = try TreeSitterLanguagePack.process(source: "FIND {test}\n", config: configObj)

```
