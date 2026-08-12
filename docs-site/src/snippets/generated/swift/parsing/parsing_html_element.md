---
id: fixture_swift_parsing_html_element
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"html\"}")
_ = try TreeSitterLanguagePack.process(source: "<div>hello</div>", config: configObj)

```
