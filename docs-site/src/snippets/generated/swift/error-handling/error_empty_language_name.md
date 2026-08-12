---
id: fixture_swift_error_empty_language_name
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

do {
    let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"\"}")
    _ = try TreeSitterLanguagePack.process(source: "hello", config: configObj)
    fatalError("expected call to fail")
} catch {
    print("Call failed as expected: \(error)")
}

```
