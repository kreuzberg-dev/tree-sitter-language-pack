---
id: fixture_swift_python_multi_import_process
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"python\"}")
_ = try TreeSitterLanguagePack.process(source: "import os\nimport sys\nfrom pathlib import Path\n\ndef main():\n    pass\n", config: configObj)

```
