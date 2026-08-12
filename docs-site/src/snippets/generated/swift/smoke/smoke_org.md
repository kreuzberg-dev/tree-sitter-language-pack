---
id: fixture_swift_smoke_org
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"org\"}")
_ = try TreeSitterLanguagePack.process(source: "* Hello\nWorld", config: configObj)

```
