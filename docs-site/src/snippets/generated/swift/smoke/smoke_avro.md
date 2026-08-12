---
id: fixture_swift_smoke_avro
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"avro\"}")
_ = try TreeSitterLanguagePack.process(source: "protocol P {\n}\n", config: configObj)

```
