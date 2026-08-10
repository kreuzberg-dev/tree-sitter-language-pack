```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"edoc\"}")
_ = try TreeSitterLanguagePack.process(source: "@doc foo\n", config: configObj)

```
