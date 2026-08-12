---
id: fixture_swift_smoke_plantuml
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"plantuml\"}")
_ = try TreeSitterLanguagePack.process(source: "@startuml\n@enduml\n", config: configObj)

```
