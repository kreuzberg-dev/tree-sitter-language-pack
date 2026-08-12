---
id: fixture_swift_smoke_embeddedtemplate
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"embeddedtemplate\"}")
_ = try TreeSitterLanguagePack.process(source: "<%= value %>", config: configObj)

```
