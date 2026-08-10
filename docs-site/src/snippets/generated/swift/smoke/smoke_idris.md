```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"idris\"}")
_ = try TreeSitterLanguagePack.process(source: "module Main", config: configObj)

```
