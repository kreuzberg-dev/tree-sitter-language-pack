---
id: fixture_swift_smoke_jai
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"jai\"}")
_ = try TreeSitterLanguagePack.process(source: "x", config: configObj)

```
