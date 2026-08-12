---
id: fixture_swift_smoke_xcompose
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"xcompose\"}")
_ = try TreeSitterLanguagePack.process(source: "<Multi_key> <a> : \"a\"", config: configObj)

```
