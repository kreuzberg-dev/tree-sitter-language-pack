```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"xquery\"}")
_ = try TreeSitterLanguagePack.process(source: "1\n", config: configObj)

```
