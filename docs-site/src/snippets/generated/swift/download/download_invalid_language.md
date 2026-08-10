```swift title="Swift"
import TreeSitterLanguagePack

do {
    _ = try TreeSitterLanguagePack.download(names: ["zzz_definitely_not_a_real_language_xyz"])
    fatalError("expected call to fail")
} catch {
    print("Call failed as expected: \(error)")
}

```
