```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"arduino\"}")
_ = try TreeSitterLanguagePack.process(source: "void setup() {}", config: configObj)

```
