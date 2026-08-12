---
id: fixture_swift_data_extraction_nginx_directives
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"data_extraction\":true,\"language\":\"nginx\"}")
_ = try TreeSitterLanguagePack.process(source: "worker_processes 4;\nerror_log /var/log/nginx/error.log;\n", config: configObj)

```
