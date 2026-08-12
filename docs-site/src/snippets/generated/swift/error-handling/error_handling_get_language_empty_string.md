---
id: fixture_swift_error_handling_get_language_empty_string
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

do {
    _ = try TreeSitterLanguagePack.getLanguage(name: "")
    fatalError("expected call to fail")
} catch {
    print("Call failed as expected: \(error)")
}

```
