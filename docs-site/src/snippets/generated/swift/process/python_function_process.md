```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"python\"}")
_ = try TreeSitterLanguagePack.process(source: "def greet(name):\n    return f'Hello, {name}!'\n", config: configObj)

```
