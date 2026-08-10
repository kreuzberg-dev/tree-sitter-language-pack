```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"c3\"}")
_ = try TreeSitterLanguagePack.process(source: "x", config: configObj)

```
