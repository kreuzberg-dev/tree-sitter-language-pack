---
id: fixture_swift_smoke_requirements
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"requirements\"}")
_ = try TreeSitterLanguagePack.process(source: "flask>=2.0", config: configObj)

```
