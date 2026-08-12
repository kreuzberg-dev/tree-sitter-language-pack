---
id: fixture_swift_smoke_properties
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"properties\"}")
_ = try TreeSitterLanguagePack.process(source: "key=value", config: configObj)

```
