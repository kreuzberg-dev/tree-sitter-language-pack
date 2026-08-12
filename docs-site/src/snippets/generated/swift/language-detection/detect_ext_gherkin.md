---
id: fixture_swift_detect_ext_gherkin
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

_ = try TreeSitterLanguagePack.detectLanguageFromExtension(ext: "feature")

```
