---
id: fixture_swift_smoke_markdown_inline
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"markdown_inline\"}")
_ = try TreeSitterLanguagePack.process(source: "**bold** and *italic*", config: configObj)

```
