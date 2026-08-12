---
id: fixture_swift_smoke_ballerina
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"ballerina\"}")
_ = try TreeSitterLanguagePack.process(source: "public function main() {\n}\n", config: configObj)

```
