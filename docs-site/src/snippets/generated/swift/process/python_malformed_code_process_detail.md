---
id: fixture_swift_python_malformed_code_process_detail
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"diagnostics\":true,\"language\":\"python\"}")
_ = try TreeSitterLanguagePack.process(source: "def broken(\n    return\nclass", config: configObj)

```
