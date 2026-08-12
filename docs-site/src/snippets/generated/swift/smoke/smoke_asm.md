---
id: fixture_swift_smoke_asm
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"asm\"}")
_ = try TreeSitterLanguagePack.process(source: "mov eax, 1", config: configObj)

```
