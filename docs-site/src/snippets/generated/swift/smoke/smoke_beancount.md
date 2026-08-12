---
id: fixture_swift_smoke_beancount
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"beancount\"}")
_ = try TreeSitterLanguagePack.process(source: "2024-01-01 open Assets:Bank USD", config: configObj)

```
