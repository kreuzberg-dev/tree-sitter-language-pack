```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"tact\"}")
_ = try TreeSitterLanguagePack.process(source: "x", config: configObj)

```
