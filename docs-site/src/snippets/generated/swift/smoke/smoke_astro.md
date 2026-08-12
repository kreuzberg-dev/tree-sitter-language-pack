---
id: fixture_swift_smoke_astro
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"astro\"}")
_ = try TreeSitterLanguagePack.process(source: "---\n---\n<p>hello</p>", config: configObj)

```
