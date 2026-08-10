```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"pem\"}")
_ = try TreeSitterLanguagePack.process(source: "-----BEGIN CERTIFICATE-----\ndata\n-----END CERTIFICATE-----", config: configObj)

```
