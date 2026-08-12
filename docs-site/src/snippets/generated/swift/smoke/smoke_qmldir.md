---
id: fixture_swift_smoke_qmldir
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"qmldir\"}")
_ = try TreeSitterLanguagePack.process(source: "module Example", config: configObj)

```
