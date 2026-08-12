---
id: fixture_swift_smoke_soql
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"soql\"}")
_ = try TreeSitterLanguagePack.process(source: "SELECT Id FROM Account\n", config: configObj)

```
