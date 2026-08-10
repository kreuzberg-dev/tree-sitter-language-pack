```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"tlaplus\"}")
_ = try TreeSitterLanguagePack.process(source: "---- MODULE Main ----\n====", config: configObj)

```
