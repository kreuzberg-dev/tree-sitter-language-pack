---
id: fixture_swift_smoke_lalrpop
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"lalrpop\"}")
_ = try TreeSitterLanguagePack.process(source: "grammar;\n", config: configObj)

```
