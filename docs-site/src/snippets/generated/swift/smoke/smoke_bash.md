```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"bash\"}")
_ = try TreeSitterLanguagePack.process(source: "echo hello", config: configObj)

```
