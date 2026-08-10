```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"yang\"}")
_ = try TreeSitterLanguagePack.process(source: "module m {\n}\n", config: configObj)

```
