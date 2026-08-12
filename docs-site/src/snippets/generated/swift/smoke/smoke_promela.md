---
id: fixture_swift_smoke_promela
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"promela\"}")
_ = try TreeSitterLanguagePack.process(source: "init {\n}\n", config: configObj)

```
