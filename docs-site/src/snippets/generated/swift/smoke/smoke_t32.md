---
id: fixture_swift_smoke_t32
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"t32\"}")
_ = try TreeSitterLanguagePack.process(source: "PRINT 1\n", config: configObj)

```
