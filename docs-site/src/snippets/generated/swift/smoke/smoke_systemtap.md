---
id: fixture_swift_smoke_systemtap
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"systemtap\"}")
_ = try TreeSitterLanguagePack.process(source: "probe begin {}\n", config: configObj)

```
