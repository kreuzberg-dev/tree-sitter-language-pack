---
id: fixture_swift_smoke_powershell
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"powershell\"}")
_ = try TreeSitterLanguagePack.process(source: "Write-Host 'hello'", config: configObj)

```
