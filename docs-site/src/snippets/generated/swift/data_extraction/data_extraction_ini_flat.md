---
id: fixture_swift_data_extraction_ini_flat
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"data_extraction\":true,\"language\":\"ini\"}")
_ = try TreeSitterLanguagePack.process(source: "host=localhost\nport=8080\n", config: configObj)

```
