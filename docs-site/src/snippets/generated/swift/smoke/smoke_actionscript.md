---
id: fixture_swift_smoke_actionscript
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"actionscript\"}")
_ = try TreeSitterLanguagePack.process(source: "var x:int = 1;", config: configObj)

```
