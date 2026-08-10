```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"v\"}")
_ = try TreeSitterLanguagePack.process(source: "fn main() {}", config: configObj)

```
