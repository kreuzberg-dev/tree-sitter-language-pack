---
id: fixture_swift_smoke_styled
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"styled\"}")
_ = try TreeSitterLanguagePack.process(source: "color: red;\n", config: configObj)

```
