---
id: fixture_swift_smoke_udev
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"udev\"}")
_ = try TreeSitterLanguagePack.process(source: "ACTION==\"add\", KERNEL==\"sd*\"", config: configObj)

```
