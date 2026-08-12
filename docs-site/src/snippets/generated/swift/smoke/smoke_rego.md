---
id: fixture_swift_smoke_rego
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"rego\"}")
_ = try TreeSitterLanguagePack.process(source: "package main\ndefault allow = false", config: configObj)

```
