```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"scala\"}")
_ = try TreeSitterLanguagePack.process(source: "object Main", config: configObj)

```
