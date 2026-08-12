---
id: fixture_swift_smoke_pem
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"pem\"}")
_ = try TreeSitterLanguagePack.process(source: "-----BEGIN CERTIFICATE-----\ndata\n-----END CERTIFICATE-----", config: configObj)

```
