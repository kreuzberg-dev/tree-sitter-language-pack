---
id: fixture_swift_smoke_rshtml
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"rshtml\"}")
_ = try TreeSitterLanguagePack.process(source: "<p>hi</p>\n", config: configObj)

```
