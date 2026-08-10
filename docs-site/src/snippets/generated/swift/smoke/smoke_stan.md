```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"stan\"}")
_ = try TreeSitterLanguagePack.process(source: "x", config: configObj)

```
