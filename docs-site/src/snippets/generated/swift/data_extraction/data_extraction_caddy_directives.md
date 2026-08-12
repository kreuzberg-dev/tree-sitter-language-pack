---
id: fixture_swift_data_extraction_caddy_directives
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"data_extraction\":true,\"language\":\"caddy\"}")
_ = try TreeSitterLanguagePack.process(source: "localhost\nroot * /var/www\nfile_server\n", config: configObj)

```
