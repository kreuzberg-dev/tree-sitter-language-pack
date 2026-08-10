```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"fusion\"}")
_ = try TreeSitterLanguagePack.process(source: "foo = 1\n", config: configObj)

```
