---
id: fixture_swift_smoke_luap
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"luap\"}")
_ = try TreeSitterLanguagePack.process(source: "[a-z]+", config: configObj)

```
