---
id: fixture_swift_smoke_smithy
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"smithy\"}")
_ = try TreeSitterLanguagePack.process(source: "namespace example\nstring MyString", config: configObj)

```
