```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"json5\"}")
_ = try TreeSitterLanguagePack.process(source: "x", config: configObj)

```
