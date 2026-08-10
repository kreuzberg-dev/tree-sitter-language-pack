```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"data_extraction\":true,\"language\":\"kdl\"}")
_ = try TreeSitterLanguagePack.process(source: "server {\n  host \"localhost\"\n  port 8080\n}\n", config: configObj)

```
