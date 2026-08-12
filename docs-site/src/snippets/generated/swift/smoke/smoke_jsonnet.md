---
id: fixture_swift_smoke_jsonnet
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"jsonnet\"}")
_ = try TreeSitterLanguagePack.process(source: "{ key: 'value' }", config: configObj)

```
