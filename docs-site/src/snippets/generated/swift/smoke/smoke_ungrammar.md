---
id: fixture_swift_smoke_ungrammar
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"ungrammar\"}")
_ = try TreeSitterLanguagePack.process(source: "Root = Item*\nItem = 'token'", config: configObj)

```
