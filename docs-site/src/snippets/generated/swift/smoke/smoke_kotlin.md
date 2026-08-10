```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"kotlin\"}")
_ = try TreeSitterLanguagePack.process(source: "fun main() {}", config: configObj)

```
