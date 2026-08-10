```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"d2\"}")
_ = try TreeSitterLanguagePack.process(source: "a -> b\n", config: configObj)

```
