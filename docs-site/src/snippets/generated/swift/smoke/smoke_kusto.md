```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"kusto\"}")
_ = try TreeSitterLanguagePack.process(source: "T | count\n", config: configObj)

```
