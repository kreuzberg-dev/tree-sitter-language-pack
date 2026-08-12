---
id: fixture_swift_smoke_csharp
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"csharp\"}")
_ = try TreeSitterLanguagePack.process(source: "class Main {}", config: configObj)

```
