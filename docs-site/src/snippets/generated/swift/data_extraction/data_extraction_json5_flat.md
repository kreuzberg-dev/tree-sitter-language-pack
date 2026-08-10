```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"data_extraction\":true,\"language\":\"json5\"}")
_ = try TreeSitterLanguagePack.process(source: "{\n  host: \"localhost\",\n  port: 8080,\n}\n", config: configObj)

```
