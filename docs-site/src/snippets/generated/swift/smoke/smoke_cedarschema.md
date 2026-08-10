```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"cedarschema\"}")
_ = try TreeSitterLanguagePack.process(source: "entity User;", config: configObj)

```
