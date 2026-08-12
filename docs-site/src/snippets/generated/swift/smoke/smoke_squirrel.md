---
id: fixture_swift_smoke_squirrel
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"squirrel\"}")
_ = try TreeSitterLanguagePack.process(source: "function main() {}", config: configObj)

```
