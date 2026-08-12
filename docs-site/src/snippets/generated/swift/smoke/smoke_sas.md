---
id: fixture_swift_smoke_sas
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"sas\"}")
_ = try TreeSitterLanguagePack.process(source: "data _null_;\nrun;\n", config: configObj)

```
