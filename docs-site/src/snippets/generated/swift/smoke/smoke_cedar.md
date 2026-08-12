---
id: fixture_swift_smoke_cedar
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"cedar\"}")
_ = try TreeSitterLanguagePack.process(source: "permit(principal, action, resource);", config: configObj)

```
