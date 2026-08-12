---
id: fixture_swift_detect_content_python_shebang
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

_ = try TreeSitterLanguagePack.detectLanguageFromContent(content: "#!/usr/bin/env python3\npass")

```
