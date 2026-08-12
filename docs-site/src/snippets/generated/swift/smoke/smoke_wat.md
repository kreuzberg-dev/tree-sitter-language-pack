---
id: fixture_swift_smoke_wat
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"wat\"}")
_ = try TreeSitterLanguagePack.process(source: "(module)", config: configObj)

```
