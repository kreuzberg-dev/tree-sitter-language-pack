---
id: fixture_swift_smoke_func
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"func\"}")
_ = try TreeSitterLanguagePack.process(source: "() recv_internal() {}", config: configObj)

```
