```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"purescript\"}")
_ = try TreeSitterLanguagePack.process(source: "module Main where", config: configObj)

```
