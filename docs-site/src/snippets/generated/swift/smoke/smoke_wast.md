```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"wast\"}")
_ = try TreeSitterLanguagePack.process(source: "(module)", config: configObj)

```
