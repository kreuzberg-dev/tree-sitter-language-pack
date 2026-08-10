```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"org\"}")
_ = try TreeSitterLanguagePack.process(source: "* Hello\nWorld", config: configObj)

```
