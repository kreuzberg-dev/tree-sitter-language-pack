```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"haskell_persistent\"}")
_ = try TreeSitterLanguagePack.process(source: "Person\n  name String\n", config: configObj)

```
