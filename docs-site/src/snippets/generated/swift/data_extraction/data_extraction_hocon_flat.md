```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"data_extraction\":true,\"language\":\"hocon\"}")
_ = try TreeSitterLanguagePack.process(source: "host = \"localhost\"\nport = 8080\n", config: configObj)

```
