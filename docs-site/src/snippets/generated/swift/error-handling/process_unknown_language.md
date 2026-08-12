---
id: fixture_swift_process_unknown_language
language: swift
target: swift
level: typecheck
requires: []
side_effect: safe
---

```swift title="Swift"
import TreeSitterLanguagePack

do {
    let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"nonexistent_xyz\"}")
    _ = try TreeSitterLanguagePack.process(source: "x = 1", config: configObj)
    fatalError("expected call to fail")
} catch {
    print("Call failed as expected: \(error)")
}

```
