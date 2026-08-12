---
id: fixture_swift_smoke_ispc
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"ispc\"}")
_ = try TreeSitterLanguagePack.process(source: "export void main() {}", config: configObj)

```
