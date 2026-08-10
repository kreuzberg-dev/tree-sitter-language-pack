```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"ziggy_schema\"}")
_ = try TreeSitterLanguagePack.process(source: "x", config: configObj)

```
