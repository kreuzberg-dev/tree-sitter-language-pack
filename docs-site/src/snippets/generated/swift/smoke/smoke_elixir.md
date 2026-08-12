---
id: fixture_swift_smoke_elixir
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"elixir\"}")
_ = try TreeSitterLanguagePack.process(source: "IO.puts(\"hello\")", config: configObj)

```
