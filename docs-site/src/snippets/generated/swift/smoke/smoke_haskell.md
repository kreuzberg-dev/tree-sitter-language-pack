---
id: fixture_swift_smoke_haskell
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"haskell\"}")
_ = try TreeSitterLanguagePack.process(source: "main = putStrLn \"hello\"", config: configObj)

```
