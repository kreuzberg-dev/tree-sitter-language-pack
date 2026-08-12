---
id: fixture_swift_smoke_idl
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"idl\"}")
_ = try TreeSitterLanguagePack.process(source: "module M {\n};\n", config: configObj)

```
