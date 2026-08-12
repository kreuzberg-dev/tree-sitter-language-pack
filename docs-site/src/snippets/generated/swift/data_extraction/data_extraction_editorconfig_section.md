---
id: fixture_swift_data_extraction_editorconfig_section
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"data_extraction\":true,\"language\":\"editorconfig\"}")
_ = try TreeSitterLanguagePack.process(source: "[*.rs]\nindent_style = space\nindent_size = 4\n", config: configObj)

```
