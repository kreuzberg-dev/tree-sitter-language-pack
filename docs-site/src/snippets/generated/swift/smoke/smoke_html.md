```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"html\"}")
_ = try TreeSitterLanguagePack.process(source: "<p>hello</p>", config: configObj)

```
