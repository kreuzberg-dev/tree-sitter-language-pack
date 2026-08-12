---
id: fixture_swift_smoke_ada
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"ada\"}")
_ = try TreeSitterLanguagePack.process(source: "procedure Main is begin null; end Main;", config: configObj)

```
