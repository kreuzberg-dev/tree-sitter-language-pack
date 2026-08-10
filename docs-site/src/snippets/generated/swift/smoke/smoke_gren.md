```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"gren\"}")
_ = try TreeSitterLanguagePack.process(source: "module Main exposing (..)", config: configObj)

```
