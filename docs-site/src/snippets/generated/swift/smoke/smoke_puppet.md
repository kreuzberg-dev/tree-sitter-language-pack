```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"puppet\"}")
_ = try TreeSitterLanguagePack.process(source: "notify { 'hello': }", config: configObj)

```
