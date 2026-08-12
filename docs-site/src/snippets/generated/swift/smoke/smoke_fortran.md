---
id: fixture_swift_smoke_fortran
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"fortran\"}")
_ = try TreeSitterLanguagePack.process(source: "program main\nend program main", config: configObj)

```
