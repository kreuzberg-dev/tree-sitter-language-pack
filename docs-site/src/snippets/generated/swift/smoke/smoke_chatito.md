```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"chatito\"}")
_ = try TreeSitterLanguagePack.process(source: "%[greeting]\n    hello", config: configObj)

```
