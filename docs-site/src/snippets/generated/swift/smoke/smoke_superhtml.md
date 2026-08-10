```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"superhtml\"}")
_ = try TreeSitterLanguagePack.process(source: "x", config: configObj)

```
