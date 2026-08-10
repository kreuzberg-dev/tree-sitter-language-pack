```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"t32\"}")
_ = try TreeSitterLanguagePack.process(source: "PRINT 1\n", config: configObj)

```
