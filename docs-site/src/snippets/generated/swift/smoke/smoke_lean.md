---
id: fixture_swift_smoke_lean
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"lean\"}")
_ = try TreeSitterLanguagePack.process(source: "def main : IO Unit := pure ()", config: configObj)

```
