```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"data_extraction\":true,\"language\":\"properties\"}")
_ = try TreeSitterLanguagePack.process(source: "server.host=localhost\nserver.port=8080\n", config: configObj)

```
