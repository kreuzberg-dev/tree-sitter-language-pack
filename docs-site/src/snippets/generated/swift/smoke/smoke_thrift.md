```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"thrift\"}")
_ = try TreeSitterLanguagePack.process(source: "service HelloService {}", config: configObj)

```
