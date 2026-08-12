---
id: fixture_swift_smoke_purescript
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"purescript\"}")
_ = try TreeSitterLanguagePack.process(source: "module Main where", config: configObj)

```
