---
id: fixture_swift_smoke_m68k
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"m68k\"}")
_ = try TreeSitterLanguagePack.process(source: " move.l d0,d1\n", config: configObj)

```
