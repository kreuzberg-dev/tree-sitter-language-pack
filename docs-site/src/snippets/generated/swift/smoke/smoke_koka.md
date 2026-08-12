---
id: fixture_swift_smoke_koka
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"koka\"}")
_ = try TreeSitterLanguagePack.process(source: "fun main()\n  1\n", config: configObj)

```
