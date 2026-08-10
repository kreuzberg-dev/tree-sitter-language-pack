```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"re2c\"}")
_ = try TreeSitterLanguagePack.process(source: "/*!re2c\n  [a-z]+ { return; }\n*/", config: configObj)

```
