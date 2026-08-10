```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"moonbit\"}")
_ = try TreeSitterLanguagePack.process(source: "fn main {\n}\n", config: configObj)

```
