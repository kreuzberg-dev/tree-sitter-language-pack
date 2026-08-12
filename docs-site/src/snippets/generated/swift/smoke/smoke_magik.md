---
id: fixture_swift_smoke_magik
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"magik\"}")
_ = try TreeSitterLanguagePack.process(source: "_method object.hello\n_endmethod", config: configObj)

```
