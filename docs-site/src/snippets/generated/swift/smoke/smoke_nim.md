---
id: fixture_swift_smoke_nim
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"nim\"}")
_ = try TreeSitterLanguagePack.process(source: "echo \"hello\"", config: configObj)

```
