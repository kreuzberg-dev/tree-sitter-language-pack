```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"menhir\"}")
_ = try TreeSitterLanguagePack.process(source: "%token EOF\n%%\n", config: configObj)

```
