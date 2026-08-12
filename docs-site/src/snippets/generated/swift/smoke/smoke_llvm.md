---
id: fixture_swift_smoke_llvm
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"llvm\"}")
_ = try TreeSitterLanguagePack.process(source: "define i32 @main() { ret i32 0 }", config: configObj)

```
