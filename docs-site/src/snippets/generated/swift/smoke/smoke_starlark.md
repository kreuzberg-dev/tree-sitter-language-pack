```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"starlark\"}")
_ = try TreeSitterLanguagePack.process(source: "def hello(): pass", config: configObj)

```
