---
id: fixture_swift_smoke_hack
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"hack\"}")
_ = try TreeSitterLanguagePack.process(source: "<?hh\nfunction main(): void {}", config: configObj)

```
