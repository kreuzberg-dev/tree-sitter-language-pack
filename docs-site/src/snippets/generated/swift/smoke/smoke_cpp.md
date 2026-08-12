---
id: fixture_swift_smoke_cpp
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"cpp\"}")
_ = try TreeSitterLanguagePack.process(source: "int main() { return 0; }", config: configObj)

```
