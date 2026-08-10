```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"git_rebase\"}")
_ = try TreeSitterLanguagePack.process(source: "x", config: configObj)

```
