```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"typescript\"}")
_ = try TreeSitterLanguagePack.process(source: "import { readFile } from 'fs';\n\nfunction greet(name: string): string {\n    return `Hello, ${name}!`;\n}\n", config: configObj)

```
