---
id: fixture_swift_smoke_gstlaunch
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"gstlaunch\"}")
_ = try TreeSitterLanguagePack.process(source: "fakesrc ! fakesink", config: configObj)

```
