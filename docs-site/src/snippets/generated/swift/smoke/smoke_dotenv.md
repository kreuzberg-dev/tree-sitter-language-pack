```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"dotenv\"}")
_ = try TreeSitterLanguagePack.process(source: "KEY=value\n", config: configObj)

```
