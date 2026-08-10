```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"c\"}")
_ = try TreeSitterLanguagePack.process(source: "int main() { return 0; }", config: configObj)

```
