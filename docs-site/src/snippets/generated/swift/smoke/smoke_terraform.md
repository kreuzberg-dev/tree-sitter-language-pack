---
id: fixture_swift_smoke_terraform
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"terraform\"}")
_ = try TreeSitterLanguagePack.process(source: "resource \"null_resource\" \"main\" {}", config: configObj)

```
