```swift title="Swift"
import TreeSitterLanguagePack

do {
    let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"nonexistent_xyz\"}")
    _ = try TreeSitterLanguagePack.process(source: "", config: configObj)
    fatalError("expected call to fail")
} catch {
    print("Call failed as expected: \(error)")
}

```
