```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"data_extraction\":true,\"language\":\"yaml\"}")
_ = try TreeSitterLanguagePack.process(source: "server:\n  host: localhost\n  port: 8080\n", config: configObj)

```
