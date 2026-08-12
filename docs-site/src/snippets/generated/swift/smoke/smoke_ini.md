---
id: fixture_swift_smoke_ini
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"ini\"}")
_ = try TreeSitterLanguagePack.process(source: "[section]\nkey = value", config: configObj)

```
