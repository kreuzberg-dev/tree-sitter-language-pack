---
id: fixture_swift_smoke_tablegen
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"tablegen\"}")
_ = try TreeSitterLanguagePack.process(source: "def Hello : Base {}", config: configObj)

```
