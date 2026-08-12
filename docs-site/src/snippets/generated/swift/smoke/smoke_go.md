---
id: fixture_swift_smoke_go
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"go\"}")
_ = try TreeSitterLanguagePack.process(source: "package main", config: configObj)

```
