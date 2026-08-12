---
id: fixture_swift_smoke_cpon
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"cpon\"}")
_ = try TreeSitterLanguagePack.process(source: "{\"key\": 1}", config: configObj)

```
