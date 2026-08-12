---
id: fixture_swift_smoke_xresources
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"xresources\"}")
_ = try TreeSitterLanguagePack.process(source: "*.foreground: #ffffff\n", config: configObj)

```
