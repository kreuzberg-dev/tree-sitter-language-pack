---
id: fixture_swift_smoke_postscript
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"postscript\"}")
_ = try TreeSitterLanguagePack.process(source: "/hello { (Hello) show } def", config: configObj)

```
