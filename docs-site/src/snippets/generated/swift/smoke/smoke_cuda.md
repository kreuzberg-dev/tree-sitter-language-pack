---
id: fixture_swift_smoke_cuda
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"cuda\"}")
_ = try TreeSitterLanguagePack.process(source: "__global__ void kernel() {}", config: configObj)

```
