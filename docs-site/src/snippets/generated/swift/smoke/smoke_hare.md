---
id: fixture_swift_smoke_hare
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"hare\"}")
_ = try TreeSitterLanguagePack.process(source: "export fn main() void = void;", config: configObj)

```
