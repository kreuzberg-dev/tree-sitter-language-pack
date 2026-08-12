---
id: fixture_swift_smoke_vala
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"vala\"}")
_ = try TreeSitterLanguagePack.process(source: "class Foo {\n}\n", config: configObj)

```
