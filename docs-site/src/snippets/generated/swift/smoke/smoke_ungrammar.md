```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"ungrammar\"}")
_ = try TreeSitterLanguagePack.process(source: "Root = Item*\nItem = 'token'", config: configObj)

```
