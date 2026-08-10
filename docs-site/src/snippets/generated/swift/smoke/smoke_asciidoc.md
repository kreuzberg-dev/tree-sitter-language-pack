```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"asciidoc\"}")
_ = try TreeSitterLanguagePack.process(source: "= Title\n\nParagraph.", config: configObj)

```
