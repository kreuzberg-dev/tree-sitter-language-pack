---
id: fixture_swift_detect_path_dotfile
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

_ = try TreeSitterLanguagePack.detectLanguageFromPath(path: ".gitignore")

```
