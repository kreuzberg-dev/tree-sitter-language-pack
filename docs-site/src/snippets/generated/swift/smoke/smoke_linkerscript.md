---
id: fixture_swift_smoke_linkerscript
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"linkerscript\"}")
_ = try TreeSitterLanguagePack.process(source: "SECTIONS { .text : { *(.text) } }", config: configObj)

```
