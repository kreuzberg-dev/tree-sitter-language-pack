```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"scfg\"}")
_ = try TreeSitterLanguagePack.process(source: "key value\n", config: configObj)

```
