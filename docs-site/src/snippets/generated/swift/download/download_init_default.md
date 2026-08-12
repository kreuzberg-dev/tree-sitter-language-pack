---
id: fixture_swift_download_init_default
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.packConfigFromJson("{}")
try TreeSitterLanguagePack.init_(config: configObj)

```
