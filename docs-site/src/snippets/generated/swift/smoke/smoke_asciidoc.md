---
id: fixture_swift_smoke_asciidoc
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"asciidoc\"}")
_ = try TreeSitterLanguagePack.process(source: "= Title\n\nParagraph.", config: configObj)

```
