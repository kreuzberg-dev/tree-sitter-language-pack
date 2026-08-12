---
id: fixture_swift_smoke_matlab
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"matlab\"}")
_ = try TreeSitterLanguagePack.process(source: "function y = hello(x)\ny = x;\nend", config: configObj)

```
