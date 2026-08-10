```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"meson\"}")
_ = try TreeSitterLanguagePack.process(source: "project('hello', 'c')", config: configObj)

```
