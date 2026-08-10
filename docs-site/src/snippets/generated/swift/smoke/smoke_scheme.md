```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"scheme\"}")
_ = try TreeSitterLanguagePack.process(source: "(define x 1)", config: configObj)

```
