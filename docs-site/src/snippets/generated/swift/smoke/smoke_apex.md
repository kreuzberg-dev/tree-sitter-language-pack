---
id: fixture_swift_smoke_apex
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"apex\"}")
_ = try TreeSitterLanguagePack.process(source: "public class Main {}", config: configObj)

```
