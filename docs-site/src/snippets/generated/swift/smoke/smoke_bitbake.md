---
id: fixture_swift_smoke_bitbake
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"bitbake\"}")
_ = try TreeSitterLanguagePack.process(source: "DESCRIPTION = \"hello\"", config: configObj)

```
