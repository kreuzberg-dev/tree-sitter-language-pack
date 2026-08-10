```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"graphql\"}")
_ = try TreeSitterLanguagePack.process(source: "type Query { hello: String }", config: configObj)

```
