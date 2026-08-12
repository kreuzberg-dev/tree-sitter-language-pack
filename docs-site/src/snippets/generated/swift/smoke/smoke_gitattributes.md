---
id: fixture_swift_smoke_gitattributes
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"gitattributes\"}")
_ = try TreeSitterLanguagePack.process(source: "*.txt text", config: configObj)

```
