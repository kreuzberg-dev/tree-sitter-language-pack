---
id: fixture_swift_process_python_docstrings
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"docstrings\":true,\"language\":\"python\"}")
_ = try TreeSitterLanguagePack.process(source: "def greet(name):\n    \"\"\"Say hello to someone.\"\"\"\n    return f\"Hello {name}\"\n", config: configObj)

```
