---
id: fixture_swift_download_configure_custom_dir
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.packConfigFromJson("{\"cache_dir\":\"/tmp/tslp_test_cache\"}")
try TreeSitterLanguagePack.configure(config: configObj)

```
