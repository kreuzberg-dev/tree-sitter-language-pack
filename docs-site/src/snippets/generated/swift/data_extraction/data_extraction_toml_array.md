```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"data_extraction\":true,\"language\":\"toml\"}")
_ = try TreeSitterLanguagePack.process(source: "ports = [8080, 8081, 8082]\n", config: configObj)

```
