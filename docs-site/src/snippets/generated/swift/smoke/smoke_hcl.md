```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"hcl\"}")
_ = try TreeSitterLanguagePack.process(source: "variable \"name\" { type = string }", config: configObj)

```
