```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"gdshader\"}")
_ = try TreeSitterLanguagePack.process(source: "x", config: configObj)

```
