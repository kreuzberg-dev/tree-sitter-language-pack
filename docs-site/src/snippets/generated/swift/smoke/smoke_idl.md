```swift title="Swift"
import TreeSitterLanguagePack

let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"idl\"}")
_ = try TreeSitterLanguagePack.process(source: "module M {\n};\n", config: configObj)

```
