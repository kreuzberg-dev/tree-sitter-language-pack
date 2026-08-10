```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"data_extraction\":true,\"language\":\"csv\"}")
_ = try TreeSitterLanguagePack.process(source: "x,y,z\n", config: configObj)

```
