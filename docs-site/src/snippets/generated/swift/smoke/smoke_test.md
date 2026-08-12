---
id: fixture_swift_smoke_test
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"test\"}")
_ = try TreeSitterLanguagePack.process(source: "===========\nTest\n===========\n---\n(node)", config: configObj)

```
