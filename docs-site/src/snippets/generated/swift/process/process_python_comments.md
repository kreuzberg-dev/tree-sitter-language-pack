---
id: fixture_swift_process_python_comments
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"comments\":true,\"language\":\"python\"}")
_ = try TreeSitterLanguagePack.process(source: "# This is a comment\n# Another comment\ndef hello():\n    # inline comment\n    pass\n", config: configObj)

```
