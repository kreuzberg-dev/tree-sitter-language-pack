```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"firrtl\"}")
_ = try TreeSitterLanguagePack.process(source: "circuit Main :", config: configObj)

```
