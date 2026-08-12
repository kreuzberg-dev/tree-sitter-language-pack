---
id: fixture_swift_smoke_haml
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"haml\"}")
_ = try TreeSitterLanguagePack.process(source: "%p hello\n", config: configObj)

```
