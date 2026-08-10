```swift title="Swift"
import TreeSitterLanguagePack

do {
    let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"\"}")
    _ = try TreeSitterLanguagePack.process(source: "x = 1", config: configObj)
    fatalError("expected call to fail")
} catch {
    print("Call failed as expected: \(error)")
}

```
