---
id: fixture_swift_smoke_cobol
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"cobol\"}")
_ = try TreeSitterLanguagePack.process(source: "       IDENTIFICATION DIVISION.\n       PROGRAM-ID. HELLO.", config: configObj)

```
