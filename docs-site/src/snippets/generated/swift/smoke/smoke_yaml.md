---
id: fixture_swift_smoke_yaml
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"yaml\"}")
_ = try TreeSitterLanguagePack.process(source: "key: value", config: configObj)

```
