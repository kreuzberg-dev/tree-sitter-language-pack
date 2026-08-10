```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"sas\"}")
_ = try TreeSitterLanguagePack.process(source: "data _null_;\nrun;\n", config: configObj)

```
