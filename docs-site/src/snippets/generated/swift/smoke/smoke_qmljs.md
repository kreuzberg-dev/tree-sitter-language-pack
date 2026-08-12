---
id: fixture_swift_smoke_qmljs
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"qmljs\"}")
_ = try TreeSitterLanguagePack.process(source: "import QtQuick 2.0\nItem {}", config: configObj)

```
