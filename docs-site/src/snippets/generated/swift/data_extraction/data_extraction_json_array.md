```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"data_extraction\":true,\"language\":\"json\"}")
_ = try TreeSitterLanguagePack.process(source: "[1, 2, 3]", config: configObj)

```
