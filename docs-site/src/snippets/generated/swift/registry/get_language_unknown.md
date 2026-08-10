```swift title="Swift"
import TreeSitterLanguagePack

do {
    _ = try TreeSitterLanguagePack.getLanguage(name: "nonexistent_xyz")
    fatalError("expected call to fail")
} catch {
    print("Call failed as expected: \(error)")
}

```
