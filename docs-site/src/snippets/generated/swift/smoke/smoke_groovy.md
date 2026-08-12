---
id: fixture_swift_smoke_groovy
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"groovy\"}")
_ = try TreeSitterLanguagePack.process(source: "def x = 1", config: configObj)

```
