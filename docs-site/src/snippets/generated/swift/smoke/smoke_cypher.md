---
id: fixture_swift_smoke_cypher
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"cypher\"}")
_ = try TreeSitterLanguagePack.process(source: "MATCH (n) RETURN n\n", config: configObj)

```
