---
id: fixture_swift_smoke_awk
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"awk\"}")
_ = try TreeSitterLanguagePack.process(source: "BEGIN { print \"hello\" }", config: configObj)

```
