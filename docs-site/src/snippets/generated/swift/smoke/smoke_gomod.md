---
id: fixture_swift_smoke_gomod
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"gomod\"}")
_ = try TreeSitterLanguagePack.process(source: "module example.com/hello\n\ngo 1.21", config: configObj)

```
