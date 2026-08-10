```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"poe_filter\"}")
_ = try TreeSitterLanguagePack.process(source: "x", config: configObj)

```
