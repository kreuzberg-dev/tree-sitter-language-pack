---
id: fixture_swift_smoke_starlark
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"starlark\"}")
_ = try TreeSitterLanguagePack.process(source: "def hello(): pass", config: configObj)

```
