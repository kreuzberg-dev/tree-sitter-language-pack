```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"lalrpop\"}")
_ = try TreeSitterLanguagePack.process(source: "grammar;\n", config: configObj)

```
