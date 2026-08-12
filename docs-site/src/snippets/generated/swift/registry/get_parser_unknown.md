---
id: fixture_swift_get_parser_unknown
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

do {
    _ = try TreeSitterLanguagePack.getParser(name: "nonexistent_xyz")
    fatalError("expected call to fail")
} catch {
    print("Call failed as expected: \(error)")
}

```
