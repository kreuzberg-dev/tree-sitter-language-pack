---
id: fixture_swift_smoke_abnf
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"abnf\"}")
_ = try TreeSitterLanguagePack.process(source: "a = \"b\"\r\n", config: configObj)

```
