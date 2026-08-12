---
id: fixture_swift_smoke_tsv
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"tsv\"}")
_ = try TreeSitterLanguagePack.process(source: "a\tb\tc\n1\t2\t3", config: configObj)

```
