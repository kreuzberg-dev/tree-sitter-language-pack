---
id: fixture_swift_smoke_ql
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"ql\"}")
_ = try TreeSitterLanguagePack.process(source: "x", config: configObj)

```
