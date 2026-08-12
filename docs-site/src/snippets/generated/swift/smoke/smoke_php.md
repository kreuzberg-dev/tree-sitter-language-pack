---
id: fixture_swift_smoke_php
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"php\"}")
_ = try TreeSitterLanguagePack.process(source: "<?php echo 'hello'; ?>", config: configObj)

```
