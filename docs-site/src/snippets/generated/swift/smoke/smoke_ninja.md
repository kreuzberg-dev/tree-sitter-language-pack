---
id: fixture_swift_smoke_ninja
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"ninja\"}")
_ = try TreeSitterLanguagePack.process(source: "rule cc\n  command = cc $in -o $out", config: configObj)

```
