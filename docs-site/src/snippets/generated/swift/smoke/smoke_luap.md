```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"luap\"}")
_ = try TreeSitterLanguagePack.process(source: "[a-z]+", config: configObj)

```
