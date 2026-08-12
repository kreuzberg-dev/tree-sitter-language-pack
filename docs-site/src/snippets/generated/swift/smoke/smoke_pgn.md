---
id: fixture_swift_smoke_pgn
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"pgn\"}")
_ = try TreeSitterLanguagePack.process(source: "1. e4 e5 *", config: configObj)

```
