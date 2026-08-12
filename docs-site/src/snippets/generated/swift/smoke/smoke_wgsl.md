---
id: fixture_swift_smoke_wgsl
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"wgsl\"}")
_ = try TreeSitterLanguagePack.process(source: "@vertex fn main() -> @builtin(position) vec4f { return vec4f(); }", config: configObj)

```
