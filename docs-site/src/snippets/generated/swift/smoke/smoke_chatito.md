---
id: fixture_swift_smoke_chatito
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"chatito\"}")
_ = try TreeSitterLanguagePack.process(source: "%[greeting]\n    hello", config: configObj)

```
