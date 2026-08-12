---
id: fixture_swift_smoke_diff
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"diff\"}")
_ = try TreeSitterLanguagePack.process(source: "--- a/file\n+++ b/file\n@@ -1 +1 @@\n-old\n+new", config: configObj)

```
