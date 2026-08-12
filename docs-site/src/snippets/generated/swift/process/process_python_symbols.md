---
id: fixture_swift_process_python_symbols
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"python\",\"symbols\":true}")
_ = try TreeSitterLanguagePack.process(source: "MY_CONST = 42\ndef helper(): pass\nclass Widget: pass\n", config: configObj)

```
