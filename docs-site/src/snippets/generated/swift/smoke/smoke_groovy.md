```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"groovy\"}")
_ = try TreeSitterLanguagePack.process(source: "def x = 1", config: configObj)

```
