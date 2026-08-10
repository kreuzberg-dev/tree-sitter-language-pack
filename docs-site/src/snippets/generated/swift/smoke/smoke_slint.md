```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"slint\"}")
_ = try TreeSitterLanguagePack.process(source: "export component Foo {}\n", config: configObj)

```
