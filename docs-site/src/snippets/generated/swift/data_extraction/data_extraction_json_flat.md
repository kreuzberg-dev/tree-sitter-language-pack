---
id: fixture_swift_data_extraction_json_flat
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"data_extraction\":true,\"language\":\"json\"}")
_ = try TreeSitterLanguagePack.process(source: "{\"host\": \"localhost\", \"port\": 8080}", config: configObj)

```
