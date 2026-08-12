---
id: fixture_swift_download_multiple_languages
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

_ = try TreeSitterLanguagePack.download(names: ["python", "rust"])

```
