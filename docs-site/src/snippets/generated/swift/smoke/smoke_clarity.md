---
id: fixture_swift_smoke_clarity
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"clarity\"}")
_ = try TreeSitterLanguagePack.process(source: "(define-public (hello) (ok true))", config: configObj)

```
