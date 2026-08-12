---
id: fixture_swift_smoke_markdown
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"markdown\"}")
_ = try TreeSitterLanguagePack.process(source: "# Hello\n\nWorld", config: configObj)

```
