---
id: fixture_swift_smoke_jsdoc
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"jsdoc\"}")
_ = try TreeSitterLanguagePack.process(source: "/** @param {string} name */", config: configObj)

```
