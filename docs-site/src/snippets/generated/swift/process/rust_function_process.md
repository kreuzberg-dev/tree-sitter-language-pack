---
id: fixture_swift_rust_function_process
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"rust\"}")
_ = try TreeSitterLanguagePack.process(source: "fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n", config: configObj)

```
