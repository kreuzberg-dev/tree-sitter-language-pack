```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"flatbuffers\"}")
_ = try TreeSitterLanguagePack.process(source: "table Foo {}\n", config: configObj)

```
