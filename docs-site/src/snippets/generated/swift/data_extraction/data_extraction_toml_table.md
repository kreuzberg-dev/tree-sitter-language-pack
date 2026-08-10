```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"data_extraction\":true,\"language\":\"toml\"}")
_ = try TreeSitterLanguagePack.process(source: "[server]\nhost = \"localhost\"\nport = 8080\n", config: configObj)

```
