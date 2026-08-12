---
id: fixture_swift_smoke_leo
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"leo\"}")
_ = try TreeSitterLanguagePack.process(source: "program test.aleo {\n}\n", config: configObj)

```
