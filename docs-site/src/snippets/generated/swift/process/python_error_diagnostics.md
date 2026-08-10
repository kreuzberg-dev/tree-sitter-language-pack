```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"diagnostics\":true,\"language\":\"python\"}")
_ = try TreeSitterLanguagePack.process(source: "def broken(\n    pass\n", config: configObj)

```
