---
id: fixture_swift_smoke_gn
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"gn\"}")
_ = try TreeSitterLanguagePack.process(source: "group(\"hello\") {}", config: configObj)

```
