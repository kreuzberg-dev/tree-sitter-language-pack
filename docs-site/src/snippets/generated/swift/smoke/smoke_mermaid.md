---
id: fixture_swift_smoke_mermaid
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"mermaid\"}")
_ = try TreeSitterLanguagePack.process(source: "graph TD\nA --> B", config: configObj)

```
