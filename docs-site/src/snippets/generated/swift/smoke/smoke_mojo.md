```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"mojo\"}")
_ = try TreeSitterLanguagePack.process(source: "x", config: configObj)

```
