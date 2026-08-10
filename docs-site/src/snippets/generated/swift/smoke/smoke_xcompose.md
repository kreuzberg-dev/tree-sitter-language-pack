```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"xcompose\"}")
_ = try TreeSitterLanguagePack.process(source: "<Multi_key> <a> : \"a\"", config: configObj)

```
