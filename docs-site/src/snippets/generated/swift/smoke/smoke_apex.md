```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"apex\"}")
_ = try TreeSitterLanguagePack.process(source: "public class Main {}", config: configObj)

```
