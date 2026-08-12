---
id: fixture_swift_smoke_solidity
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"solidity\"}")
_ = try TreeSitterLanguagePack.process(source: "pragma solidity ^0.8.0;\ncontract Main {}", config: configObj)

```
