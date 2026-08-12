---
id: fixture_swift_smoke_kotlin
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"kotlin\"}")
_ = try TreeSitterLanguagePack.process(source: "fun main() {}", config: configObj)

```
