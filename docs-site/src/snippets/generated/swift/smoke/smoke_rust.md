---
id: fixture_swift_smoke_rust
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"rust\"}")
_ = try TreeSitterLanguagePack.process(source: "fn main() {}", config: configObj)

```
