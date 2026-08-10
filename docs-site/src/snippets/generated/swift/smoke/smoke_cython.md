```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"cython\"}")
_ = try TreeSitterLanguagePack.process(source: "x = 1\n", config: configObj)

```
