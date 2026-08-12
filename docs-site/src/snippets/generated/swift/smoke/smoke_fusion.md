---
id: fixture_swift_smoke_fusion
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"fusion\"}")
_ = try TreeSitterLanguagePack.process(source: "foo = 1\n", config: configObj)

```
