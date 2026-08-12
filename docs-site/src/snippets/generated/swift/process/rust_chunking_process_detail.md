---
id: fixture_swift_rust_chunking_process_detail
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"chunk_max_size\":30,\"language\":\"rust\"}")
_ = try TreeSitterLanguagePack.process(source: "fn alpha() {}\n\nfn beta() {}\n\nfn gamma() {}\n\nfn delta() {}\n", config: configObj)

```
