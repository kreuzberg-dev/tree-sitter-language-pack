---
id: fixture_swift_smoke_llvm_mir
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"llvm_mir\"}")
_ = try TreeSitterLanguagePack.process(source: "---\nname: foo\n...\n", config: configObj)

```
