---
id: fixture_swift_smoke_rtf
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"rtf\"}")
_ = try TreeSitterLanguagePack.process(source: "{\\rtf1 hello}", config: configObj)

```
