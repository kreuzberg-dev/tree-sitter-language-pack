---
id: fixture_swift_smoke_capnp
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"capnp\"}")
_ = try TreeSitterLanguagePack.process(source: "@0xabcdef1234567890;", config: configObj)

```
