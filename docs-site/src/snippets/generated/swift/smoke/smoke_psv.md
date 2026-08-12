---
id: fixture_swift_smoke_psv
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"psv\"}")
_ = try TreeSitterLanguagePack.process(source: "a|b|c\n1|2|3", config: configObj)

```
