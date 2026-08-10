```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"ziggy\"}")
_ = try TreeSitterLanguagePack.process(source: "x", config: configObj)

```
