---
id: fixture_swift_smoke_spicedb
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"spicedb\"}")
_ = try TreeSitterLanguagePack.process(source: "definition user {}\n", config: configObj)

```
