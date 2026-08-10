```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"julia\"}")
_ = try TreeSitterLanguagePack.process(source: "function main() end", config: configObj)

```
