```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"magik\"}")
_ = try TreeSitterLanguagePack.process(source: "_method object.hello\n_endmethod", config: configObj)

```
