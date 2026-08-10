```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"al\"}")
_ = try TreeSitterLanguagePack.process(source: "x", config: configObj)

```
