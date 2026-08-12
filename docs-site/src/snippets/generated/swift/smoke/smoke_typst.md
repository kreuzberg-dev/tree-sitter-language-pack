---
id: fixture_swift_smoke_typst
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"typst\"}")
_ = try TreeSitterLanguagePack.process(source: "#let x = 1", config: configObj)

```
