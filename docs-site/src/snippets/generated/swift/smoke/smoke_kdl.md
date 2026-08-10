```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"kdl\"}")
_ = try TreeSitterLanguagePack.process(source: "node \"value\"", config: configObj)

```
