```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"data_extraction\":true,\"language\":\"ini\"}")
_ = try TreeSitterLanguagePack.process(source: "[database]\nhost=localhost\nport=5432\n", config: configObj)

```
