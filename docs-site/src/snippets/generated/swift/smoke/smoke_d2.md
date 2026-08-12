---
id: fixture_swift_smoke_d2
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"d2\"}")
_ = try TreeSitterLanguagePack.process(source: "a -> b\n", config: configObj)

```
