```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"matlab\"}")
_ = try TreeSitterLanguagePack.process(source: "function y = hello(x)\ny = x;\nend", config: configObj)

```
