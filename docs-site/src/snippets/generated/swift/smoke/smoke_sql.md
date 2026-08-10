```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"sql\"}")
_ = try TreeSitterLanguagePack.process(source: "SELECT 1;", config: configObj)

```
