---
id: fixture_swift_smoke_slint
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"slint\"}")
_ = try TreeSitterLanguagePack.process(source: "export component Foo {}\n", config: configObj)

```
