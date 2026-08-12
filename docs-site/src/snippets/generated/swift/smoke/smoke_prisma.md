---
id: fixture_swift_smoke_prisma
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"prisma\"}")
_ = try TreeSitterLanguagePack.process(source: "model User { id Int @id }", config: configObj)

```
