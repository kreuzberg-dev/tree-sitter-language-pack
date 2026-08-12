---
id: fixture_swift_smoke_gleam
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"gleam\"}")
_ = try TreeSitterLanguagePack.process(source: "pub fn main() { }", config: configObj)

```
