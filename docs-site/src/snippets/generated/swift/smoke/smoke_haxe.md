---
id: fixture_swift_smoke_haxe
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"haxe\"}")
_ = try TreeSitterLanguagePack.process(source: "class Main { static function main() {} }", config: configObj)

```
