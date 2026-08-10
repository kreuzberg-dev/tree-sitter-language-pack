```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"ruby\"}")
_ = try TreeSitterLanguagePack.process(source: "puts 'hello'", config: configObj)

```
