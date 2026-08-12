---
id: fixture_swift_smoke_puppet
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"puppet\"}")
_ = try TreeSitterLanguagePack.process(source: "notify { 'hello': }", config: configObj)

```
