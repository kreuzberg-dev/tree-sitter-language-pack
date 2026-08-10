```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"gitcommit\"}")
_ = try TreeSitterLanguagePack.process(source: "feat: add feature\n\nBody text", config: configObj)

```
