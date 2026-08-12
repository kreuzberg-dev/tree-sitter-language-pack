---
id: fixture_swift_smoke_koto
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"koto\"}")
_ = try TreeSitterLanguagePack.process(source: "x = 1\n", config: configObj)

```
