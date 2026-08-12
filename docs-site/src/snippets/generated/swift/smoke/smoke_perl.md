---
id: fixture_swift_smoke_perl
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"perl\"}")
_ = try TreeSitterLanguagePack.process(source: "print 'hello';", config: configObj)

```
