---
id: fixture_swift_parsing_go_function
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"go\"}")
_ = try TreeSitterLanguagePack.process(source: "package main\nfunc main() {}", config: configObj)

```
