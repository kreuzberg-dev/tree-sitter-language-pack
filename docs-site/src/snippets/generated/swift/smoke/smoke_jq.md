---
id: fixture_swift_smoke_jq
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"jq\"}")
_ = try TreeSitterLanguagePack.process(source: ".[] | select(.key)", config: configObj)

```
