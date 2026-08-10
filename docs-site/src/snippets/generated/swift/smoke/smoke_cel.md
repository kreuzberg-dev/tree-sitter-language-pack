```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"cel\"}")
_ = try TreeSitterLanguagePack.process(source: "x", config: configObj)

```
