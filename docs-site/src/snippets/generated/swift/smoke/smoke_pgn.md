```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"pgn\"}")
_ = try TreeSitterLanguagePack.process(source: "1. e4 e5 *", config: configObj)

```
