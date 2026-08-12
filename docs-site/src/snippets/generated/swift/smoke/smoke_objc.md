---
id: fixture_swift_smoke_objc
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"objc\"}")
_ = try TreeSitterLanguagePack.process(source: "@interface Main @end", config: configObj)

```
