---
id: fixture_swift_smoke_pymanifest
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"pymanifest\"}")
_ = try TreeSitterLanguagePack.process(source: "include *.txt", config: configObj)

```
