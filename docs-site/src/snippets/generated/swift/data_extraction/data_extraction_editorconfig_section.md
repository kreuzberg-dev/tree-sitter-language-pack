```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"data_extraction\":true,\"language\":\"editorconfig\"}")
_ = try TreeSitterLanguagePack.process(source: "[*.rs]\nindent_style = space\nindent_size = 4\n", config: configObj)

```
