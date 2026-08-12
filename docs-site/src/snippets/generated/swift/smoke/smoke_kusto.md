---
id: fixture_swift_smoke_kusto
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"kusto\"}")
_ = try TreeSitterLanguagePack.process(source: "T | count\n", config: configObj)

```
