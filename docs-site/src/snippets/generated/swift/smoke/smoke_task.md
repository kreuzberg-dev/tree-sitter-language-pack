```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"task\"}")
_ = try TreeSitterLanguagePack.process(source: "todo item\n", config: configObj)

```
