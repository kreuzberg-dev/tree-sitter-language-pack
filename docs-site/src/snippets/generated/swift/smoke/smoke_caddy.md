```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"caddy\"}")
_ = try TreeSitterLanguagePack.process(source: ":8080 {\n\trespond \"Hello\"\n}", config: configObj)

```
