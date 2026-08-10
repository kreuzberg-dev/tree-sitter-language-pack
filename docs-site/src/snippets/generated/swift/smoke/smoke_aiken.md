```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"aiken\"}")
_ = try TreeSitterLanguagePack.process(source: "fn main() {\n  1\n}\n", config: configObj)

```
