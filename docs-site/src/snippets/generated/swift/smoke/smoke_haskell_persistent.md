---
id: fixture_swift_smoke_haskell_persistent
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"haskell_persistent\"}")
_ = try TreeSitterLanguagePack.process(source: "Person\n  name String\n", config: configObj)

```
