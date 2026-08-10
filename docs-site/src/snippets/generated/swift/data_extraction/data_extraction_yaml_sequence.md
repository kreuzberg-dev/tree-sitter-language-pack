```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"data_extraction\":true,\"language\":\"yaml\"}")
_ = try TreeSitterLanguagePack.process(source: "ports:\n  - 8080\n  - 8081\n", config: configObj)

```
