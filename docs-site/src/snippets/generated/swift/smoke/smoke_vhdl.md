---
id: fixture_swift_smoke_vhdl
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"vhdl\"}")
_ = try TreeSitterLanguagePack.process(source: "entity main is end main;", config: configObj)

```
