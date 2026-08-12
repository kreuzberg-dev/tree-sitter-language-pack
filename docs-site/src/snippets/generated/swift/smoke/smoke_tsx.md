---
id: fixture_swift_smoke_tsx
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"tsx\"}")
_ = try TreeSitterLanguagePack.process(source: "const App = () => <div />;", config: configObj)

```
