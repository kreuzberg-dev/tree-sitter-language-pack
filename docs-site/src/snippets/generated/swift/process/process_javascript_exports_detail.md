```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"javascript\"}")
_ = try TreeSitterLanguagePack.process(source: "export function greet(name) {\n  return `Hello ${name}`;\n}\n\nexport const VERSION = '1.0';\n", config: configObj)

```
