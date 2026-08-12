---
id: fixture_swift_smoke_janet
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"janet\"}")
_ = try TreeSitterLanguagePack.process(source: "(print \"hello\")", config: configObj)

```
