---
id: fixture_swift_smoke_sparql
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"sparql\"}")
_ = try TreeSitterLanguagePack.process(source: "SELECT ?s WHERE { ?s ?p ?o }", config: configObj)

```
