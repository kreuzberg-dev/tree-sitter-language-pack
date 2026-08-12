---
id: fixture_swift_smoke_twig
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"twig\"}")
_ = try TreeSitterLanguagePack.process(source: "{{ variable }}", config: configObj)

```
