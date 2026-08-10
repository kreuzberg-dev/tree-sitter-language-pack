```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"vento\"}")
_ = try TreeSitterLanguagePack.process(source: "hello\n", config: configObj)

```
