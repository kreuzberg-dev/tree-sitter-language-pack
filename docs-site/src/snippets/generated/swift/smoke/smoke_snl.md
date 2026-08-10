```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"snl\"}")
_ = try TreeSitterLanguagePack.process(source: "program test\n", config: configObj)

```
