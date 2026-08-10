```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"agda\"}")
_ = try TreeSitterLanguagePack.process(source: "module Main where", config: configObj)

```
