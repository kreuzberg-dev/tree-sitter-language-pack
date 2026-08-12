---
id: fixture_swift_smoke_re2c
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"re2c\"}")
_ = try TreeSitterLanguagePack.process(source: "/*!re2c\n  [a-z]+ { return; }\n*/", config: configObj)

```
