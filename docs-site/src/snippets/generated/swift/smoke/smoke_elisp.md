---
id: fixture_swift_smoke_elisp
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"elisp\"}")
_ = try TreeSitterLanguagePack.process(source: "(defun hello () (message \"hello\"))", config: configObj)

```
