---
id: fixture_swift_smoke_make
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"make\"}")
_ = try TreeSitterLanguagePack.process(source: "all:\n\techo hello", config: configObj)

```
