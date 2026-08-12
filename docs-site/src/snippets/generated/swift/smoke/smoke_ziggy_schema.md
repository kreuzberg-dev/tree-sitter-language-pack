---
id: fixture_swift_smoke_ziggy_schema
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"ziggy_schema\"}")
_ = try TreeSitterLanguagePack.process(source: "x", config: configObj)

```
