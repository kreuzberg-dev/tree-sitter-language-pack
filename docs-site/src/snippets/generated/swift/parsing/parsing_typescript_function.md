```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"typescript\"}")
_ = try TreeSitterLanguagePack.process(source: "function greet(name: string): string { return `hi ${name}`; }", config: configObj)

```
