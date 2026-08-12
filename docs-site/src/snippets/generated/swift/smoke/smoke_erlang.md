---
id: fixture_swift_smoke_erlang
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"erlang\"}")
_ = try TreeSitterLanguagePack.process(source: "main() -> ok.", config: configObj)

```
