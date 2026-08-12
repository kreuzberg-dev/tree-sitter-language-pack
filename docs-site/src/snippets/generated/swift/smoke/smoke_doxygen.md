---
id: fixture_swift_smoke_doxygen
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"doxygen\"}")
_ = try TreeSitterLanguagePack.process(source: "/** @brief A function */", config: configObj)

```
