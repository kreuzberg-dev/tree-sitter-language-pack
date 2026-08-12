---
id: fixture_swift_smoke_uxntal
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"uxntal\"}")
_ = try TreeSitterLanguagePack.process(source: "|0100 LIT 01", config: configObj)

```
