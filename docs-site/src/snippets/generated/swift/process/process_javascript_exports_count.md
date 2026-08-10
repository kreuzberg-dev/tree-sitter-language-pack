```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"javascript\"}")
_ = try TreeSitterLanguagePack.process(source: "export function greet() { return 'hi'; }\nexport const VERSION = '1.0';\nexport default class App {}\n", config: configObj)

```
