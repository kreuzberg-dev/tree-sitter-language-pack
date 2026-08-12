---
id: fixture_swift_detect_content_no_shebang
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

_ = try TreeSitterLanguagePack.detectLanguageFromContent(content: "no shebang here")

```
