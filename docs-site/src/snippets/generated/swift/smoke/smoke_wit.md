---
id: fixture_swift_smoke_wit
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"wit\"}")
_ = try TreeSitterLanguagePack.process(source: "package example:pkg;", config: configObj)

```
