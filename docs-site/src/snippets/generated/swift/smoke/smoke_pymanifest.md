```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"pymanifest\"}")
_ = try TreeSitterLanguagePack.process(source: "include *.txt", config: configObj)

```
