---
id: fixture_swift_smoke_yang
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"yang\"}")
_ = try TreeSitterLanguagePack.process(source: "module m {\n}\n", config: configObj)

```
