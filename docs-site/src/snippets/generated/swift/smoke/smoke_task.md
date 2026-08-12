---
id: fixture_swift_smoke_task
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"task\"}")
_ = try TreeSitterLanguagePack.process(source: "todo item\n", config: configObj)

```
