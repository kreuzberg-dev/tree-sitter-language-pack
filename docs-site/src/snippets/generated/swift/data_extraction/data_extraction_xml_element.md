---
id: fixture_swift_data_extraction_xml_element
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"data_extraction\":true,\"language\":\"xml\"}")
_ = try TreeSitterLanguagePack.process(source: "<server id=\"main\"><host>localhost</host></server>", config: configObj)

```
