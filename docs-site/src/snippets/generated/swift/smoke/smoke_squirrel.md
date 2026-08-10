```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"squirrel\"}")
_ = try TreeSitterLanguagePack.process(source: "function main() {}", config: configObj)

```
