---
id: fixture_swift_smoke_fluent
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"fluent\"}")
_ = try TreeSitterLanguagePack.process(source: "hello = Hello\n", config: configObj)

```
