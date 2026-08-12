---
id: fixture_swift_data_extraction_hcl_attribute
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"data_extraction\":true,\"language\":\"hcl\"}")
_ = try TreeSitterLanguagePack.process(source: "region = \"us-east-1\"\ncount  = 3\n", config: configObj)

```
