---
id: fixture_swift_smoke_gdscript
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"gdscript\"}")
_ = try TreeSitterLanguagePack.process(source: "extends Node\nfunc _ready():\n\tpass", config: configObj)

```
