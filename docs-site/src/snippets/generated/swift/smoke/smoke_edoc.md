---
id: fixture_swift_smoke_edoc
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"edoc\"}")
_ = try TreeSitterLanguagePack.process(source: "@doc foo\n", config: configObj)

```
