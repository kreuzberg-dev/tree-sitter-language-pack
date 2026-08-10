```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"markdown_inline\"}")
_ = try TreeSitterLanguagePack.process(source: "**bold** and *italic*", config: configObj)

```
