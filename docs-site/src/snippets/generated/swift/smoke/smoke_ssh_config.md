---
id: fixture_swift_smoke_ssh_config
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"ssh_config\"}")
_ = try TreeSitterLanguagePack.process(source: "Host example\n  HostName example.com", config: configObj)

```
