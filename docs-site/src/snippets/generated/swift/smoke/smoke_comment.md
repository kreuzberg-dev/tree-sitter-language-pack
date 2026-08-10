```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"comment\"}")
_ = try TreeSitterLanguagePack.process(source: "Review: handle edge case", config: configObj)

```
