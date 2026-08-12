---
id: fixture_swift_smoke_hcl
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"hcl\"}")
_ = try TreeSitterLanguagePack.process(source: "variable \"name\" { type = string }", config: configObj)

```
