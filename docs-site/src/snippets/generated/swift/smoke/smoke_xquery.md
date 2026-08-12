---
id: fixture_swift_smoke_xquery
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"xquery\"}")
_ = try TreeSitterLanguagePack.process(source: "1\n", config: configObj)

```
