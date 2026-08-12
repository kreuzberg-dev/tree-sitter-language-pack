---
id: fixture_swift_error_handling_haskell_unterminated_block_comment
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"haskell\"}")
_ = try TreeSitterLanguagePack.process(source: "{-aaaaaaaaaaaaaa aaaa}\n    {-aaa (aaaaaaaaaa [aaaaaaaaaaaaa aaa", config: configObj)

```
