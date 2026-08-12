---
id: fixture_swift_smoke_lua
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"lua\"}")
_ = try TreeSitterLanguagePack.process(source: "print('hello')", config: configObj)

```
