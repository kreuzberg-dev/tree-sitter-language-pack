```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"data_extraction\":true,\"language\":\"json\"}")
_ = try TreeSitterLanguagePack.process(source: "{\"server\": {\"host\": \"x\", \"port\": 8080}}", config: configObj)

```
