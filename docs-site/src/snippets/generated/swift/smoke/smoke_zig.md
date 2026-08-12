---
id: fixture_swift_smoke_zig
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"zig\"}")
_ = try TreeSitterLanguagePack.process(source: "pub fn main() void {}", config: configObj)

```
