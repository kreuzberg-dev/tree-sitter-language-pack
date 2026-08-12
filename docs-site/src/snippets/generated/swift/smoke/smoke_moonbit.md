---
id: fixture_swift_smoke_moonbit
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"moonbit\"}")
_ = try TreeSitterLanguagePack.process(source: "fn main {\n}\n", config: configObj)

```
