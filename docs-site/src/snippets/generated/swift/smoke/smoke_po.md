---
id: fixture_swift_smoke_po
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"po\"}")
_ = try TreeSitterLanguagePack.process(source: "msgid \"hello\"\nmsgstr \"world\"", config: configObj)

```
