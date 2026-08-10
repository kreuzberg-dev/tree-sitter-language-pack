```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"spicedb\"}")
_ = try TreeSitterLanguagePack.process(source: "definition user {}\n", config: configObj)

```
