---
id: fixture_swift_smoke_jinja2
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"jinja2\"}")
_ = try TreeSitterLanguagePack.process(source: "{{ variable }}", config: configObj)

```
