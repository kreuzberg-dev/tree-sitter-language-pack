---
id: fixture_swift_smoke_fsharp_signature
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"fsharp_signature\"}")
_ = try TreeSitterLanguagePack.process(source: "val x: int", config: configObj)

```
