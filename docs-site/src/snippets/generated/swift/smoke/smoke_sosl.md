```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"sosl\"}")
_ = try TreeSitterLanguagePack.process(source: "FIND {test}\n", config: configObj)

```
