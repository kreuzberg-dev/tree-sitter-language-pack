---
id: fixture_swift_smoke_idris
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"idris\"}")
_ = try TreeSitterLanguagePack.process(source: "module Main", config: configObj)

```
