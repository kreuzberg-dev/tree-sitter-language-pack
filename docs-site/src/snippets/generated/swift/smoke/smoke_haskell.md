```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"haskell\"}")
_ = try TreeSitterLanguagePack.process(source: "main = putStrLn \"hello\"", config: configObj)

```
