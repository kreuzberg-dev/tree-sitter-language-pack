---
id: fixture_swift_smoke_elm
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"elm\"}")
_ = try TreeSitterLanguagePack.process(source: "module Main exposing (..)", config: configObj)

```
