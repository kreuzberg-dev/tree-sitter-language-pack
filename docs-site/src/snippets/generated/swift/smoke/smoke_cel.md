---
id: fixture_swift_smoke_cel
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"cel\"}")
_ = try TreeSitterLanguagePack.process(source: "x", config: configObj)

```
