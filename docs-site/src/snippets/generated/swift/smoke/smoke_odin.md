---
id: fixture_swift_smoke_odin
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"odin\"}")
_ = try TreeSitterLanguagePack.process(source: "package main", config: configObj)

```
