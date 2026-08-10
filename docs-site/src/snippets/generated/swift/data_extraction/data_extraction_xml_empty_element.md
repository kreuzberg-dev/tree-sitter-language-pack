```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"data_extraction\":true,\"language\":\"xml\"}")
_ = try TreeSitterLanguagePack.process(source: "<br/>", config: configObj)

```
