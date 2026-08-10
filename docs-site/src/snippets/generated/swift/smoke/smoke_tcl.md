```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"tcl\"}")
_ = try TreeSitterLanguagePack.process(source: "puts hello", config: configObj)

```
