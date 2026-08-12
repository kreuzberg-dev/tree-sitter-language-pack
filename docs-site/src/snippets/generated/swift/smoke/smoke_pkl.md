---
id: fixture_swift_smoke_pkl
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"pkl\"}")
_ = try TreeSitterLanguagePack.process(source: "name = \"hello\"", config: configObj)

```
