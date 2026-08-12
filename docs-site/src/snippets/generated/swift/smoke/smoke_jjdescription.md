---
id: fixture_swift_smoke_jjdescription
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"jjdescription\"}")
_ = try TreeSitterLanguagePack.process(source: "commit message\n", config: configObj)

```
