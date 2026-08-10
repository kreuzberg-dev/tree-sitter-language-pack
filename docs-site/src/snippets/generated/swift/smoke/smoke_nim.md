```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"nim\"}")
_ = try TreeSitterLanguagePack.process(source: "echo \"hello\"", config: configObj)

```
