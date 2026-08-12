---
id: fixture_swift_smoke_snl
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"snl\"}")
_ = try TreeSitterLanguagePack.process(source: "program test\n", config: configObj)

```
