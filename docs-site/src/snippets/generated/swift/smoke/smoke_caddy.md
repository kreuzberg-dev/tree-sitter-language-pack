---
id: fixture_swift_smoke_caddy
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"caddy\"}")
_ = try TreeSitterLanguagePack.process(source: ":8080 {\n\trespond \"Hello\"\n}", config: configObj)

```
