```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"requirements\"}")
_ = try TreeSitterLanguagePack.process(source: "flask>=2.0", config: configObj)

```
