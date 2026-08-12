---
id: fixture_swift_smoke_nqc
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"nqc\"}")
_ = try TreeSitterLanguagePack.process(source: "task main() {}", config: configObj)

```
