---
id: fixture_swift_smoke_bsl
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"bsl\"}")
_ = try TreeSitterLanguagePack.process(source: "Procedure Main() EndProcedure", config: configObj)

```
