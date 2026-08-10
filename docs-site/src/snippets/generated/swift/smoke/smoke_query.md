```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"query\"}")
_ = try TreeSitterLanguagePack.process(source: "(identifier) @name", config: configObj)

```
