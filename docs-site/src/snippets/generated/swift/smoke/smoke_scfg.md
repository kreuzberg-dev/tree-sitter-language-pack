---
id: fixture_swift_smoke_scfg
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"scfg\"}")
_ = try TreeSitterLanguagePack.process(source: "key value\n", config: configObj)

```
