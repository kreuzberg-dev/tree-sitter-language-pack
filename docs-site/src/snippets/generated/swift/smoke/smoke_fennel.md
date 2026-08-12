---
id: fixture_swift_smoke_fennel
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"fennel\"}")
_ = try TreeSitterLanguagePack.process(source: "(fn hello [] (print :hello))", config: configObj)

```
