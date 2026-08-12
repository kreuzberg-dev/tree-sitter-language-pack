---
id: fixture_swift_data_extraction_csv_single_row
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"data_extraction\":true,\"language\":\"csv\"}")
_ = try TreeSitterLanguagePack.process(source: "x,y,z\n", config: configObj)

```
