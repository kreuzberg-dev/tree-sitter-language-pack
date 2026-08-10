```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"nqc\"}")
_ = try TreeSitterLanguagePack.process(source: "task main() {}", config: configObj)

```
