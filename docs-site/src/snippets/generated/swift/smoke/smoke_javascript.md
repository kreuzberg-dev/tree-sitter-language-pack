```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"javascript\"}")
_ = try TreeSitterLanguagePack.process(source: "console.log('hello');", config: configObj)

```
