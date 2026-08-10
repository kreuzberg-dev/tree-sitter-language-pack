```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"gitattributes\"}")
_ = try TreeSitterLanguagePack.process(source: "*.txt text", config: configObj)

```
