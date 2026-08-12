---
id: fixture_swift_smoke_eiffel
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"eiffel\"}")
_ = try TreeSitterLanguagePack.process(source: "class FOO\nend\n", config: configObj)

```
