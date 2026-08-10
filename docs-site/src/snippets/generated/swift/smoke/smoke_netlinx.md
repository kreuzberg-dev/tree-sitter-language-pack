```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"netlinx\"}")
_ = try TreeSitterLanguagePack.process(source: "PROGRAM_NAME='hello'", config: configObj)

```
