---
id: fixture_swift_smoke_git_config
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"git_config\"}")
_ = try TreeSitterLanguagePack.process(source: "x", config: configObj)

```
