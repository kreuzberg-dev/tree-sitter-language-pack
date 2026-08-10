```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"postgres\"}")
_ = try TreeSitterLanguagePack.process(source: "SELECT 1;\n", config: configObj)

```
