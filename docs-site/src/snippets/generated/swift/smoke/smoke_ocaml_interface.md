---
id: fixture_swift_smoke_ocaml_interface
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"ocaml_interface\"}")
_ = try TreeSitterLanguagePack.process(source: "val x : int", config: configObj)

```
