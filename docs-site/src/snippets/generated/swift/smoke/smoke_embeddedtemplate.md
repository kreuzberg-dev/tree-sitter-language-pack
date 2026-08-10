```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"embeddedtemplate\"}")
_ = try TreeSitterLanguagePack.process(source: "<%= value %>", config: configObj)

```
