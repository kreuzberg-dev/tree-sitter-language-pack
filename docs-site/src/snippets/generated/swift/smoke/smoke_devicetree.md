---
id: fixture_swift_smoke_devicetree
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"devicetree\"}")
_ = try TreeSitterLanguagePack.process(source: "/dts-v1/;\n/ { };", config: configObj)

```
