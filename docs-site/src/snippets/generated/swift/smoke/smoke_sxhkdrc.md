---
id: fixture_swift_smoke_sxhkdrc
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"sxhkdrc\"}")
_ = try TreeSitterLanguagePack.process(source: "super + a\n\techo hi\n", config: configObj)

```
