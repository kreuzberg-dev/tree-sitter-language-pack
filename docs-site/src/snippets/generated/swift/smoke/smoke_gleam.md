```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"gleam\"}")
_ = try TreeSitterLanguagePack.process(source: "pub fn main() { }", config: configObj)

```
