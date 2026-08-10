```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"wat\"}")
_ = try TreeSitterLanguagePack.process(source: "(module)", config: configObj)

```
