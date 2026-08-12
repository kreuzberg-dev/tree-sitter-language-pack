---
id: fixture_swift_smoke_graphql
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"graphql\"}")
_ = try TreeSitterLanguagePack.process(source: "type Query { hello: String }", config: configObj)

```
