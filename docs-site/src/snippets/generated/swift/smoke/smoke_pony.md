---
id: fixture_swift_smoke_pony
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"pony\"}")
_ = try TreeSitterLanguagePack.process(source: "actor Main\n  new create(env: Env) => None", config: configObj)

```
