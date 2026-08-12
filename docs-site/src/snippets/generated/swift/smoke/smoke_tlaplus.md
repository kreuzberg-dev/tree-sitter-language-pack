---
id: fixture_swift_smoke_tlaplus
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"tlaplus\"}")
_ = try TreeSitterLanguagePack.process(source: "---- MODULE Main ----\n====", config: configObj)

```
