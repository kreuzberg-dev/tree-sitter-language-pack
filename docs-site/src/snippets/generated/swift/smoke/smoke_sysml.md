```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"sysml\"}")
_ = try TreeSitterLanguagePack.process(source: "package P {}\n", config: configObj)

```
