```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"bitbake\"}")
_ = try TreeSitterLanguagePack.process(source: "DESCRIPTION = \"hello\"", config: configObj)

```
