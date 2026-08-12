---
id: fixture_swift_smoke_kconfig
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"kconfig\"}")
_ = try TreeSitterLanguagePack.process(source: "config FOO\n\tbool \"Enable foo\"", config: configObj)

```
