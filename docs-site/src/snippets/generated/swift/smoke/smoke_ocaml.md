---
id: fixture_swift_smoke_ocaml
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"ocaml\"}")
_ = try TreeSitterLanguagePack.process(source: "let () = print_endline \"hello\"", config: configObj)

```
