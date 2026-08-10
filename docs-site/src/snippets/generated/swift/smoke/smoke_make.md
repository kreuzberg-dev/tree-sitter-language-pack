```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"make\"}")
_ = try TreeSitterLanguagePack.process(source: "all:\n\techo hello", config: configObj)

```
