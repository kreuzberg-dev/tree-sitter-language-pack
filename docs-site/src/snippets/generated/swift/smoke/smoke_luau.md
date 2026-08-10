```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"luau\"}")
_ = try TreeSitterLanguagePack.process(source: "local x: number = 1", config: configObj)

```
