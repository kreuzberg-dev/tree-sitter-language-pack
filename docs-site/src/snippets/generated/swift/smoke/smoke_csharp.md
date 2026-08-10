```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"csharp\"}")
_ = try TreeSitterLanguagePack.process(source: "class Main {}", config: configObj)

```
