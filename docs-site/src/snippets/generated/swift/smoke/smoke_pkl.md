```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"pkl\"}")
_ = try TreeSitterLanguagePack.process(source: "name = \"hello\"", config: configObj)

```
