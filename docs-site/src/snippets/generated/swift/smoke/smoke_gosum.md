---
id: fixture_swift_smoke_gosum
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"gosum\"}")
_ = try TreeSitterLanguagePack.process(source: "example.com/pkg v1.0.0 h1:abc=", config: configObj)

```
