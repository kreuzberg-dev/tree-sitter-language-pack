```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"jjdescription\"}")
_ = try TreeSitterLanguagePack.process(source: "commit message\n", config: configObj)

```
