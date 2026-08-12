---
id: fixture_swift_smoke_agda
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"agda\"}")
_ = try TreeSitterLanguagePack.process(source: "module Main where", config: configObj)

```
