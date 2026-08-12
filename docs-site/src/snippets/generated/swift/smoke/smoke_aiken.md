---
id: fixture_swift_smoke_aiken
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"aiken\"}")
_ = try TreeSitterLanguagePack.process(source: "fn main() {\n  1\n}\n", config: configObj)

```
