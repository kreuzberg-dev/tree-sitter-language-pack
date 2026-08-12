---
id: fixture_swift_smoke_yuck
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"yuck\"}")
_ = try TreeSitterLanguagePack.process(source: "(defwidget main [] (label :text \"hi\"))", config: configObj)

```
