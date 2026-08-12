---
id: fixture_swift_smoke_sysml
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"sysml\"}")
_ = try TreeSitterLanguagePack.process(source: "package P {}\n", config: configObj)

```
