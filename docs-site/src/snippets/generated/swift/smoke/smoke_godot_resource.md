---
id: fixture_swift_smoke_godot_resource
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"godot_resource\"}")
_ = try TreeSitterLanguagePack.process(source: "x", config: configObj)

```
