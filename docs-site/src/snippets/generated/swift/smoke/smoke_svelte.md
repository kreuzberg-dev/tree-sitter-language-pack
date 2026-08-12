---
id: fixture_swift_smoke_svelte
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"svelte\"}")
_ = try TreeSitterLanguagePack.process(source: "<script>let x = 1;</script>", config: configObj)

```
