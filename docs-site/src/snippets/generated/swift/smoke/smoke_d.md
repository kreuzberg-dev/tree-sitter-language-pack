---
id: fixture_swift_smoke_d
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"d\"}")
_ = try TreeSitterLanguagePack.process(source: "void main() {}", config: configObj)

```
