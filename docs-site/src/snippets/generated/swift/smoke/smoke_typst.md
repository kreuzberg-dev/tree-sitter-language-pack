```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"typst\"}")
_ = try TreeSitterLanguagePack.process(source: "#let x = 1", config: configObj)

```
