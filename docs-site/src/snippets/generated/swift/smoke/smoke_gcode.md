---
id: fixture_swift_smoke_gcode
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"gcode\"}")
_ = try TreeSitterLanguagePack.process(source: "G0 X0\n", config: configObj)

```
