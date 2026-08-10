```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"qmldir\"}")
_ = try TreeSitterLanguagePack.process(source: "module Example", config: configObj)

```
