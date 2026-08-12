---
id: fixture_swift_smoke_dockerfile
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"dockerfile\"}")
_ = try TreeSitterLanguagePack.process(source: "FROM alpine", config: configObj)

```
