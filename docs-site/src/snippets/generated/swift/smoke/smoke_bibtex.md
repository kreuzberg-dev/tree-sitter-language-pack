---
id: fixture_swift_smoke_bibtex
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"bibtex\"}")
_ = try TreeSitterLanguagePack.process(source: "@article{key, title={A}}", config: configObj)

```
