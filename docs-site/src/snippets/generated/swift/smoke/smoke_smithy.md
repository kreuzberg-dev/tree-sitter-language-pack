```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"smithy\"}")
_ = try TreeSitterLanguagePack.process(source: "namespace example\nstring MyString", config: configObj)

```
