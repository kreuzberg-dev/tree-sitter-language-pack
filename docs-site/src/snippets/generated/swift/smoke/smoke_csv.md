---
id: fixture_swift_smoke_csv
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"csv\"}")
_ = try TreeSitterLanguagePack.process(source: "a,b,c\n1,2,3", config: configObj)

```
