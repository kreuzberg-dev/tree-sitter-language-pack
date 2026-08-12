---
id: fixture_swift_smoke_picat
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"picat\"}")
_ = try TreeSitterLanguagePack.process(source: "main => true.\n", config: configObj)

```
