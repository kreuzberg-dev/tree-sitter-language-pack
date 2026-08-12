---
id: fixture_swift_smoke_hyprlang
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"hyprlang\"}")
_ = try TreeSitterLanguagePack.process(source: "general { border_size = 1 }", config: configObj)

```
