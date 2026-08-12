---
id: fixture_swift_detect_path_go_nested
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

_ = try TreeSitterLanguagePack.detectLanguageFromPath(path: "lib/server.go")

```
