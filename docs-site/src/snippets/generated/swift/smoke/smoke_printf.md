```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"printf\"}")
_ = try TreeSitterLanguagePack.process(source: "%d %s", config: configObj)

```
