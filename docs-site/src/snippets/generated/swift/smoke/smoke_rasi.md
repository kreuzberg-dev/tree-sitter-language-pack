```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"rasi\"}")
_ = try TreeSitterLanguagePack.process(source: "x", config: configObj)

```
