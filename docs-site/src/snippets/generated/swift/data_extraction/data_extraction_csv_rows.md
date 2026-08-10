```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"data_extraction\":true,\"language\":\"csv\"}")
_ = try TreeSitterLanguagePack.process(source: "a,b,c\n1,2,3\n", config: configObj)

```
