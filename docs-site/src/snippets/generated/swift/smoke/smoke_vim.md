```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"vim\"}")
_ = try TreeSitterLanguagePack.process(source: "echo 'hello'", config: configObj)

```
