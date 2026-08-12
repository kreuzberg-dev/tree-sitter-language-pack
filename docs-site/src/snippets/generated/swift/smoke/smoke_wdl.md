---
id: fixture_swift_smoke_wdl
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"wdl\"}")
_ = try TreeSitterLanguagePack.process(source: "version 1.0\n", config: configObj)

```
