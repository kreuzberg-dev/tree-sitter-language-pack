```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"properties\"}")
_ = try TreeSitterLanguagePack.process(source: "key=value", config: configObj)

```
