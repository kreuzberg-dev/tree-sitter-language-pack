```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"vb\"}")
_ = try TreeSitterLanguagePack.process(source: "Module Main\nEnd Module", config: configObj)

```
