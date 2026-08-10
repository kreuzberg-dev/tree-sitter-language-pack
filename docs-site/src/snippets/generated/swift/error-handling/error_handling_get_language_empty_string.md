```swift title="Swift"
import TreeSitterLanguagePack

do {
    _ = try TreeSitterLanguagePack.getLanguage(name: "")
    fatalError("expected call to fail")
} catch {
    print("Call failed as expected: \(error)")
}

```
