---
id: fixture_swift_smoke_latex
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"latex\"}")
_ = try TreeSitterLanguagePack.process(source: "\\documentclass{article}\n\\begin{document}\nHello\n\\end{document}", config: configObj)

```
