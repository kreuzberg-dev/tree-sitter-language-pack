```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"cpon\"}")
_ = try TreeSitterLanguagePack.process(source: "{\"key\": 1}", config: configObj)

```
