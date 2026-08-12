---
id: fixture_swift_smoke_prolog
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"prolog\"}")
_ = try TreeSitterLanguagePack.process(source: "hello :- write('hello'), nl.", config: configObj)

```
