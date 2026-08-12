---
id: fixture_swift_smoke_menhir
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"menhir\"}")
_ = try TreeSitterLanguagePack.process(source: "%token EOF\n%%\n", config: configObj)

```
