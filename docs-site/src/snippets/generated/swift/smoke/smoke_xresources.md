```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"xresources\"}")
_ = try TreeSitterLanguagePack.process(source: "*.foreground: #ffffff\n", config: configObj)

```
