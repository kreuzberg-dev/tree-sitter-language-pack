```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"applescript\"}")
_ = try TreeSitterLanguagePack.process(source: "set x to 1\n", config: configObj)

```
