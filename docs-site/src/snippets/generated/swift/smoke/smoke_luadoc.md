```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"luadoc\"}")
_ = try TreeSitterLanguagePack.process(source: "---@param name string", config: configObj)

```
