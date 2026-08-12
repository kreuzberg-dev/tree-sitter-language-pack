---
id: fixture_swift_smoke_motoko
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"motoko\"}")
_ = try TreeSitterLanguagePack.process(source: "actor {\n}\n", config: configObj)

```
