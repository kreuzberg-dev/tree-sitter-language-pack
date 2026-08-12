---
id: fixture_swift_smoke_xml
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"xml\"}")
_ = try TreeSitterLanguagePack.process(source: "<?xml version=\"1.0\"?>\n<root>hello</root>", config: configObj)

```
