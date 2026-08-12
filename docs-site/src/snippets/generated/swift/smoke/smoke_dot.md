---
id: fixture_swift_smoke_dot
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"dot\"}")
_ = try TreeSitterLanguagePack.process(source: "digraph G { A -> B; }", config: configObj)

```
