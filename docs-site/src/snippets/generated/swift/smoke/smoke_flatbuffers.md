---
id: fixture_swift_smoke_flatbuffers
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"flatbuffers\"}")
_ = try TreeSitterLanguagePack.process(source: "table Foo {}\n", config: configObj)

```
