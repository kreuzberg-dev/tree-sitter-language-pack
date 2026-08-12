---
id: fixture_swift_data_extraction_properties_dotted_key
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"data_extraction\":true,\"language\":\"properties\"}")
_ = try TreeSitterLanguagePack.process(source: "server.host=localhost\nserver.port=8080\n", config: configObj)

```
