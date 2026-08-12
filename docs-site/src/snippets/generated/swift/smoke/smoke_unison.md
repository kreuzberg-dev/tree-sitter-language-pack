---
id: fixture_swift_smoke_unison
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"unison\"}")
_ = try TreeSitterLanguagePack.process(source: "x = 1\n", config: configObj)

```
