```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"xit\"}")
_ = try TreeSitterLanguagePack.process(source: "[ ] todo\n", config: configObj)

```
