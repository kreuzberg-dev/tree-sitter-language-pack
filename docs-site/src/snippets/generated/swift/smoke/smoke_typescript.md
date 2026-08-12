---
id: fixture_swift_smoke_typescript
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"typescript\"}")
_ = try TreeSitterLanguagePack.process(source: "const x: number = 42;", config: configObj)

```
