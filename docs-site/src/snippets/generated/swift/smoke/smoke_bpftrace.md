---
id: fixture_swift_smoke_bpftrace
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"bpftrace\"}")
_ = try TreeSitterLanguagePack.process(source: "BEGIN { }\n", config: configObj)

```
