```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"capnp\"}")
_ = try TreeSitterLanguagePack.process(source: "@0xabcdef1234567890;", config: configObj)

```
