---
id: fixture_swift_smoke_racket
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"racket\"}")
_ = try TreeSitterLanguagePack.process(source: "#lang racket\n(define x 1)", config: configObj)

```
