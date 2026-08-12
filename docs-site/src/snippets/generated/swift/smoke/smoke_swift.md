---
id: fixture_swift_smoke_swift
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"swift\"}")
_ = try TreeSitterLanguagePack.process(source: "print(\"hello\")", config: configObj)

```
