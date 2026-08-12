---
id: fixture_swift_smoke_batch
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"batch\"}")
_ = try TreeSitterLanguagePack.process(source: "@echo off\necho hello", config: configObj)

```
