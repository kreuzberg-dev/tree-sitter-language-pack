---
id: fixture_swift_smoke_css
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"css\"}")
_ = try TreeSitterLanguagePack.process(source: "body { color: red; }", config: configObj)

```
