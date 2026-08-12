---
id: fixture_swift_smoke_dtd
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"dtd\"}")
_ = try TreeSitterLanguagePack.process(source: "<!ELEMENT note (body)>", config: configObj)

```
