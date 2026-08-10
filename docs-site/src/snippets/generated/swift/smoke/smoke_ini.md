```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"ini\"}")
_ = try TreeSitterLanguagePack.process(source: "[section]\nkey = value", config: configObj)

```
