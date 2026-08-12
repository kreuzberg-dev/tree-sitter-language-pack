---
id: fixture_swift_error_detect_content_empty
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

_ = try TreeSitterLanguagePack.detectLanguageFromContent(content: "")

```
