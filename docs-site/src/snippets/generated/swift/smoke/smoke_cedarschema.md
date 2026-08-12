---
id: fixture_swift_smoke_cedarschema
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"cedarschema\"}")
_ = try TreeSitterLanguagePack.process(source: "entity User;", config: configObj)

```
