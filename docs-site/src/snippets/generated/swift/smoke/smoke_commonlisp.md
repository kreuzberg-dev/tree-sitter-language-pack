---
id: fixture_swift_smoke_commonlisp
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"commonlisp\"}")
_ = try TreeSitterLanguagePack.process(source: "(defun hello () (print \"hello\"))", config: configObj)

```
