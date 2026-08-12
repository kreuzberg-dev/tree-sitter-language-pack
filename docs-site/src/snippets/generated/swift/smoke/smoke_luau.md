---
id: fixture_swift_smoke_luau
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"luau\"}")
_ = try TreeSitterLanguagePack.process(source: "local x: number = 1", config: configObj)

```
