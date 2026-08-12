---
id: fixture_swift_smoke_julia
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"julia\"}")
_ = try TreeSitterLanguagePack.process(source: "function main() end", config: configObj)

```
