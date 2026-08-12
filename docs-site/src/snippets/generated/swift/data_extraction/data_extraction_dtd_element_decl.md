---
id: fixture_swift_data_extraction_dtd_element_decl
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"data_extraction\":true,\"language\":\"dtd\"}")
_ = try TreeSitterLanguagePack.process(source: "<!ELEMENT server (host, port)>\n<!ELEMENT host (#PCDATA)>\n", config: configObj)

```
