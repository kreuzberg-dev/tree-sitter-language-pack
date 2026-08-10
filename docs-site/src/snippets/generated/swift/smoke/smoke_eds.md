```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"eds\"}")
_ = try TreeSitterLanguagePack.process(source: "x", config: configObj)

```
