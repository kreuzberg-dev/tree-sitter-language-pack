```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"ron\"}")
_ = try TreeSitterLanguagePack.process(source: "(key: \"value\")", config: configObj)

```
