---
id: fixture_swift_smoke_sql
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"sql\"}")
_ = try TreeSitterLanguagePack.process(source: "SELECT 1;", config: configObj)

```
