---
id: fixture_swift_smoke_postgres
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"postgres\"}")
_ = try TreeSitterLanguagePack.process(source: "SELECT 1;\n", config: configObj)

```
