---
id: fixture_swift_error_detect_path_empty
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

_ = try TreeSitterLanguagePack.detectLanguageFromPath(path: "")

```
