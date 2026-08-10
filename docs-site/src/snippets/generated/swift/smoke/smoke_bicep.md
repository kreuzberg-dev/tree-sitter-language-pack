```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"bicep\"}")
_ = try TreeSitterLanguagePack.process(source: "param name string", config: configObj)

```
