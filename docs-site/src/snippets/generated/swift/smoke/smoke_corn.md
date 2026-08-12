---
id: fixture_swift_smoke_corn
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"corn\"}")
_ = try TreeSitterLanguagePack.process(source: "{ key = \"value\" }", config: configObj)

```
