---
id: fixture_swift_smoke_ruby
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"ruby\"}")
_ = try TreeSitterLanguagePack.process(source: "puts 'hello'", config: configObj)

```
